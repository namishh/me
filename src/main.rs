use actix_web::{web, App, Error, HttpServer, Result, middleware, HttpResponse};
use pulldown_cmark::{Parser, Options, html, Tag, TagEnd, CodeBlockKind, Event};
use serde::{Deserialize, Serialize};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{css_for_theme_with_class_style, ClassStyle, IncludeBackground};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tera::Tera;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value as JsonValue;
use serde_yaml;

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

struct AppState {
    tera: Tera,
    highlight_css: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileNode {
    name: String,
    path: String,
    is_dir: bool,
    children: Vec<FileNode>,
}

fn build_file_tree(base: &Path, relative: &Path) -> Vec<FileNode> {
    let full_path = base.join(relative);
    let mut nodes = Vec::new();

    if let Ok(entries) = fs::read_dir(&full_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            if file_name.starts_with('.') {
                continue;
            }

            let is_dir = path.is_dir();
            let rel_path = relative.join(&file_name);
            let path_str = rel_path.to_string_lossy().replace('\\', "/");

            if is_dir {
                let children = build_file_tree(base, &rel_path);
                nodes.push(FileNode {
                    name: file_name,
                    path: path_str.to_string(),
                    is_dir,
                    children,
                });
            } else if path.extension().map_or(false, |ext| ext == "md") {
                nodes.push(FileNode {
                    name: file_name,
                    path: path_str.to_string()[..path_str.len() - 3].to_string(),
                    is_dir,
                    children: Vec::new(),
                });
            }
        }
    }

    nodes.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    nodes
}

fn markdown_to_html(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);

    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();
    let mut highlighter: Option<HighlightLines> = None;
    let mut events = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    _ => String::new(),
                };
                let syntax = SYNTAX_SET.find_syntax_by_token(&code_lang)
                    .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
                highlighter = Some(HighlightLines::new(
                    syntax,
                    &THEME_SET.themes["base16-ocean.dark"],
                ));
                code_content.clear();
            }
            Event::Text(text) if in_code_block => {
                code_content.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) if in_code_block => {
                in_code_block = false;
                if let Some(mut h) = highlighter.take() {
                    let mut output = String::new();
                    for line in LinesWithEndings::from(&code_content) {
                        let regions = h.highlight_line(line, &SYNTAX_SET).unwrap();
                        let html_line = syntect::html::styled_line_to_highlighted_html(
                            &regions,
                            IncludeBackground::No,
                        ).expect("Failed to convert line to HTML");
                        output.push_str(&html_line);
                    }
                    events.push(Event::Html(format!("<pre><code>{}</code></pre>", output).into()));
                }
            }
            _ => {
                if in_code_block {
                    if let Event::Text(text) = event {
                        code_content.push_str(&text);
                    }
                } else {
                    events.push(event);
                }
            }
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    html_output
}

async fn index(
    app_state: web::Data<AppState>,
    _: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let base_path = Path::new("content");
    let file_tree = build_file_tree(base_path, Path::new(""));

    let mut context = tera::Context::new();
    context.insert("file_tree", &file_tree);

    let html = app_state.tera
        .render("index.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

async fn view_markdown(
    app_state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    let path_param = &path.0;
    let base_path = PathBuf::from("content");
    let mut file_path = base_path.join(path_param);

    // Check if the path is a file
    if !file_path.is_file() {
        // Try appending .md
        let md_path = file_path.with_extension("md");
        if md_path.is_file() {
            file_path = md_path;
        } else {
            // Check if it's a directory and has an index.md
            if file_path.is_dir() {
                let index_path = file_path.join("index.md");
                if index_path.is_file() {
                    file_path = index_path;
                } else {
                    // Render 404 template
                    let context = tera::Context::new();
                    let html = app_state.tera
                        .render("404.html", &context)
                        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
                    return Ok(HttpResponse::NotFound()
                        .content_type("text/html")
                        .body(html));
                }
            } else {
                // Render 404 template
                let context = tera::Context::new();
                let html = app_state.tera
                    .render("404.html", &context)
                    .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
                return Ok(HttpResponse::NotFound()
                    .content_type("text/html")
                    .body(html));
            }
        }
    }

    let raw_content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(_) => {
            let context = tera::Context::new();
            let html = app_state.tera
                .render("404.html", &context)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
            return Ok(HttpResponse::NotFound()
                .content_type("text/html")
                .body(html));
        }
    };

    let re = Regex::new(r"(?s)^-{3,}\s*\n(.*?)\n-{3,}\s*\n(.*)").unwrap();
    let (mut frontmatter, body) = if let Some(caps) = re.captures(&raw_content) {
        let yaml_str = caps.get(1).unwrap().as_str();
        let content_part = caps.get(2).unwrap().as_str().trim_start();
        let yaml: JsonValue = serde_yaml::from_str(yaml_str).unwrap_or_else(|e| {
            eprintln!("Frontmatter parse error in {}: {}", path_param, e);
            JsonValue::Null
        });
        (yaml, content_part)
    } else {
        eprintln!("No frontmatter found in {}", path_param);
        (JsonValue::Null, raw_content.as_str())
    };

    if let JsonValue::Object(ref mut map) = frontmatter {
        if !map.contains_key("title") {
            eprintln!("Missing title in frontmatter for {}", path_param);
            map.insert("title".to_string(), JsonValue::String("Untitled".to_string()));
        }
    } else {
        frontmatter = JsonValue::Object({
            let mut map = serde_json::Map::new();
            map.insert("title".to_string(), JsonValue::String("Untitled".to_string()));
            map
        });
    }

    let content_html = markdown_to_html(body);

    let mut context = tera::Context::new();
    
    if let JsonValue::Object(fm_map) = frontmatter {
        for (key, value) in fm_map {
            context.insert(key, &value);
        }
    }

    let base_path = Path::new("content");
    let file_tree = build_file_tree(base_path, Path::new(""));
    context.insert("file_tree", &file_tree);
    context.insert("content", &content_html);
    context.insert("file_path", &path_param);
    context.insert("highlight_css", &app_state.highlight_css);

    let html = app_state.tera
        .render("view.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let highlight_css = css_for_theme_with_class_style(
        &THEME_SET.themes["base16-ocean.dark"],
        ClassStyle::Spaced,
    ).unwrap();

    HttpServer::new(move || {
        let tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Template parsing error(s): {}", e);
                std::process::exit(1);
            }
        };

        let app_state = AppState {
            tera,
            highlight_css: highlight_css.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/{path:.*}").route(web::get().to(view_markdown)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}