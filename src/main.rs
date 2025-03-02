use actix_web::{web, App, Error, HttpServer, Result, middleware, HttpResponse};
use pulldown_cmark::{Parser, Options, html, Tag, TagEnd, CodeBlockKind, Event};
use serde::{Deserialize, Serialize};
use inkjet::{Highlighter, Language, formatter};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use tera::Tera;
use regex::Regex;
use serde_json::Value as JsonValue;
use serde_yaml;
use std::env;

struct AppState {
    tera: Tera,
    highlighter: Highlighter,
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

fn extract_language_and_filename(info_string: &str) -> (Option<String>, Option<String>) {
    let parts: Vec<&str> = info_string.split_whitespace().collect();
    
    let language = if !parts.is_empty() {
        Some(parts[0].to_string())
    } else {
        None
    };
    
    // title="example.html"
    let filename = parts.iter()
        .find(|part| part.starts_with("title="))
        .and_then(|part| {
            let eq_pos = part.find('=').unwrap_or(0);
            if eq_pos < part.len() - 1 {
                let value = &part[eq_pos + 1..];
                if (value.starts_with('"') && value.ends_with('"')) || 
                   (value.starts_with('\'') && value.ends_with('\'')) {
                    Some(value[1..value.len()-1].to_string())
                } else {
                    Some(value.to_string())
                }
            } else {
                None
            }
        });
    
    (language, filename)
}

fn get_inkjet_language(lang_str: &str) -> Option<Language> {
    match lang_str.to_lowercase().as_str() {
        "rust" => Some(Language::Rust),
        "javascript" | "js" => Some(Language::Javascript),
        "typescript" | "ts" => Some(Language::Typescript),
        "python" | "py" => Some(Language::Python),
        "css" => Some(Language::Css),
        "html" => Some(Language::Html),
        "lua" => Some(Language::Lua),
        "jsx" => Some(Language::Jsx),
        "tsx" => Some(Language::Tsx),
        "zig" => Some(Language::Zig),
        _ => None,
    }
}

fn parse_highlighting_info(info_string: &str) -> (HashSet<usize>, HashSet<usize>, HashSet<usize>) {
    let mut del_lines = HashSet::new();
    let mut add_lines = HashSet::new();
    let mut h_lines = HashSet::new();

    let del_re = Regex::new(r"del=\{([^}]+)\}").ok();
    let add_re = Regex::new(r"add=\{([^}]+)\}").ok();
    let h_re = Regex::new(r"\{([^}]+)\}").ok();

    let parse_ranges = |range_str: &str | -> HashSet<usize> {
        let mut result = HashSet::new();
        for part in range_str.split(',') {
            let part = part.trim();
            if part.contains('-') {
                let range: Vec<&str> = part.split('-').collect();
                if range.len() == 2 {
                    if let (Ok(start), Ok(end)) = (range[0].trim().parse::<usize>(), range[1].trim().parse::<usize>()) {
                        for i in start..=end {
                            result.insert(i);
                        }
                    }
                }
            } else if let Ok(num) = part.parse::<usize>() {
                result.insert(num);
            }
        }
        result
    };

    if let Some(ref del_re) = del_re {
        if let Some(captures) = del_re.captures(info_string) {
            if let Some(ranges) = captures.get(1) {
                del_lines = parse_ranges(ranges.as_str());
            }
        }
    }

   if let Some(ref add_re) = add_re {
        if let Some(captures) = add_re.captures(info_string) {
            if let Some(ranges) = captures.get(1) {
                add_lines = parse_ranges(ranges.as_str());
            }
        }
    }

    if let Some(ref h_re) = h_re {
        for captures in h_re.captures_iter(info_string) {
            if let Some(range_match) = captures.get(1) {
                let full_match = captures.get(0).unwrap().as_str();
                if !full_match.starts_with("del=") && !full_match.starts_with("add=") {
                    h_lines = parse_ranges(range_match.as_str());
                }
            }
        }
    }
    

    (del_lines, add_lines, h_lines)
}

fn markdown_to_html(content: &str, highlighter: &mut Highlighter) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);

    let mut in_code_block = false;
    let mut code_content = String::new();
    let mut current_language = None;
    let mut current_filename = None;
    let mut current_heading: Option<(u8, Vec<Event>)> = None; // (level, inner_events)

    let mut current_highlighting: (HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
        (HashSet::new(), HashSet::new(), HashSet::new()); // del, add, highlight

    let mut events = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                let lang_info = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    _ => String::new(),
                };
                
                let (lang, filename) = extract_language_and_filename(&lang_info);
                current_language = lang;
                current_filename = filename;
                current_highlighting = parse_highlighting_info(&lang_info);
                code_content.clear();
            }
            Event::Text(text) if in_code_block => {
                code_content.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) if in_code_block => {
                in_code_block = false;
                
                let highlighted_html = if let Some(lang_str) = &current_language {
                    if let Some(inkjet_lang) = get_inkjet_language(lang_str) {
                        match highlighter.highlight_to_string(inkjet_lang, &formatter::Html, &code_content) {
                            Ok(html) => html,
                            Err(e) => {
                                eprintln!("Error highlighting code: {}", e);
                                htmlescape::encode_minimal(&code_content)
                            }
                        }
                    } else {
                        htmlescape::encode_minimal(&code_content)
                    }
                } else {
                    htmlescape::encode_minimal(&code_content)
                };
                
                let lines: Vec<&str> = highlighted_html.lines().collect();
                
                let total_lines = lines.len();
                let width_needed = if total_lines > 0 {
                    total_lines.to_string().len()
                } else {
                    1
                };
                
                let (del_lines, add_lines, highlight_lines) = &current_highlighting;
                let line_numbered_html = lines
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        let line_num = i + 1;
                        
                        let mut line_class = String::new();
                        if del_lines.contains(&line_num) {
                            line_class = " class=\"highlight-del\"".to_string();
                        } else if add_lines.contains(&line_num) {
                            line_class = " class=\"highlight-add\"".to_string();
                        } else if highlight_lines.contains(&line_num) {
                            line_class = " class=\"highlight\"".to_string();
                        }
                        
                        format!(
                            "<span{line_class}><span class=\"line-number\">{:0width$}</span>{}</span>", 
                            line_num, 
                            line,
                            width = width_needed,
                            line_class = line_class
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                
                let code_html = if let Some(filename) = &current_filename {
                    format!(
                        "<div class=\"code-block\"><div class=\"code-filename\">{}</div><pre><code>{}</code></pre></div>", 
                        filename, 
                        line_numbered_html
                    )
                } else {
                    format!("<pre><code>{}</code></pre>", line_numbered_html)
                };
                
                events.push(Event::Html(code_html.into()));
                
                current_language = None;
                current_filename = None;
                current_highlighting = (HashSet::new(), HashSet::new(), HashSet::new());
            }
            Event::Start(Tag::Heading { level, .. }) => {
                current_heading = match level {
                    pulldown_cmark::HeadingLevel::H1 =>  Some((1, Vec::new())),
                    pulldown_cmark::HeadingLevel::H2 => Some((2, Vec::new())),
                    pulldown_cmark::HeadingLevel::H3 => Some((3, Vec::new())),
                    pulldown_cmark::HeadingLevel::H4 => Some((4, Vec::new())),
                    _ => None
                };
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, inner_events)) = current_heading.take() {
                    let mut text_content = String::new();
                    for e in &inner_events {
                        if let Event::Text(t) = e {
                            text_content.push_str(t);
                        }
                    }
                    
                    let slug = text_content
                        .trim()
                        .to_lowercase()
                        .replace(' ', "-")
                        .replace(|c: char| !c.is_alphanumeric() && c != '-', "");

                    let mut inner_html = String::new();
                    html::push_html(&mut inner_html, inner_events.into_iter());

                    let heading_html = format!("<h{} id=\"{}\">{}</h{}>", 
                        level, slug, inner_html, level);
                    events.push(Event::Html(heading_html.into()));
                }
            }
            _ => {
                if in_code_block {
                    if let Event::Text(text) = event {
                        code_content.push_str(&text);
                    }
                } else if let Some((_, ref mut inner_events)) = current_heading {
                    inner_events.push(event);
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
        let md_path = file_path.with_extension("md");
        if md_path.is_file() {
            file_path = md_path;
        } else {
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

    // Highlight code and process markdown
    let mut highlighter = app_state.highlighter.clone();
    let content_html = markdown_to_html(body, &mut highlighter);

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

    let html = app_state.tera
        .render("view.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let highlighter = Highlighter::new();

    let mut address = "127.0.0.1:8080";

    if let Ok(arg) = env::var("ENVIRONMENT") {
        if arg == "PRODUCTION" {
            address = "0.0.0.0:8080"
        }
    }

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
            highlighter: highlighter.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/{path:.*}").route(web::get().to(view_markdown)))
    })
    .bind(address)?
    .run()
    .await
}