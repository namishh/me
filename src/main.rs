use actix_web::{web, App, Error, HttpServer, Result, middleware, Responder, HttpResponse};
use pulldown_cmark::{Parser, Options, html, Tag, TagEnd, CodeBlockKind, Event};
use serde::{Deserialize, Serialize};
use inkjet::{Highlighter, Language, formatter};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use tera::Tera;
use serde_json::Value as JsonValue;
use serde_yaml;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use image::{DynamicImage, ImageBuffer, ImageEncoder, Rgba};
use imageproc::drawing;
use ab_glyph::{FontRef, PxScale};
use std::time::Duration;
use tokio::time;

struct AppState {
    tera: Tera,
    highlighter: Arc<Mutex<Highlighter>>,
    file_tree: Arc<Vec<FileNode>>,
    title_font: Arc<FontRef<'static>>,
    path_font: Arc<FontRef<'static>>,
    avatar: Arc<tokio::sync::RwLock<Option<DynamicImage>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
                let mut name = String::new();
                let default_name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if let Ok(file) = File::open(&path) {
                    let reader = BufReader::new(file);
                    let mut in_frontmatter = false;
                    let mut found_title = false;

                    for line in reader.lines().filter_map(Result::ok) {
                        let trimmed_line = line.trim();

                        if trimmed_line == "---" {
                            if in_frontmatter {
                                break;
                            } else {
                                in_frontmatter = true;
                                continue;
                            }
                        }

                        if in_frontmatter {
                            if let Some((key, value)) = trimmed_line.split_once(':') {
                                let key = key.trim();
                                if key == "title" {
                                    name = value.trim().to_string();
                                    found_title = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !found_title || name.is_empty() {
                        name = default_name.clone();
                    }
                } else {
                    name = default_name.clone();
                }

                let trimmed_path = if path_str.ends_with(".md") {
                    path_str[..path_str.len() - 3].to_string()
                } else {
                    path_str.clone()
                };

                nodes.push(FileNode {
                    name,
                    path: trimmed_path,
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

fn get_file_tree(file_tree: &Arc<Vec<FileNode>>) -> Vec<FileNode> {
    file_tree.as_ref().clone()
}

fn extract_language_and_filename(info_string: &str) -> (Option<String>, Option<String>) {
    let parts: Vec<&str> = info_string.split_whitespace().collect();
    
    let language = if !parts.is_empty() {
        Some(parts[0].to_string())
    } else {
        None
    };
    
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

lazy_static! {
    static ref LANGUAGE_MAP: HashMap<&'static str, Language> = {
        let mut m = HashMap::new();
        m.insert("rust", Language::Rust);
        m.insert("javascript", Language::Javascript);
        m.insert("js", Language::Javascript);
        m.insert("typescript", Language::Typescript);
        m.insert("ts", Language::Typescript);
        m.insert("python", Language::Python);
        m.insert("py", Language::Python);
        m.insert("css", Language::Css);
        m.insert("html", Language::Html);
        m.insert("lua", Language::Lua);
        m.insert("jsx", Language::Jsx);
        m.insert("tsx", Language::Tsx);
        m.insert("zig", Language::Zig);
        m
    };
}

fn get_inkjet_language(lang_str: &str) -> Option<Language> {
    LANGUAGE_MAP.get(lang_str.to_lowercase().as_str()).cloned()
}

fn parse_highlighting_info(info_string: &str) -> (HashSet<usize>, HashSet<usize>, HashSet<usize>) {
    let mut del_lines = HashSet::new();
    let mut add_lines = HashSet::new();
    let mut h_lines = HashSet::new();

    lazy_static! {
        static ref DEL_RE: Regex = Regex::new(r"del=\{([^}]+)\}").unwrap();
        static ref ADD_RE: Regex = Regex::new(r"add=\{([^}]+)\}").unwrap();
        static ref H_RE: Regex = Regex::new(r"\{([^}]+)\}").unwrap();
    }

    let parse_ranges = |range_str: &str| -> HashSet<usize> {
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

    if let Some(captures) = DEL_RE.captures(info_string) {
        if let Some(ranges) = captures.get(1) {
            del_lines = parse_ranges(ranges.as_str());
        }
    }

    if let Some(captures) = ADD_RE.captures(info_string) {
        if let Some(ranges) = captures.get(1) {
            add_lines = parse_ranges(ranges.as_str());
        }
    }

    for captures in H_RE.captures_iter(info_string) {
        if let Some(range_match) = captures.get(1) {
            let full_match = captures.get(0).unwrap().as_str();
            if !full_match.starts_with("del=") && !full_match.starts_with("add=") {
                h_lines = parse_ranges(range_match.as_str());
            }
        }
    }

    (del_lines, add_lines, h_lines)
}

fn markdown_to_html(content: &str, highlighter: &Mutex<Highlighter>) -> (String, Vec<(u8, String, String)>) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);

    let mut in_code_block = false;
    let mut code_content = String::new();
    let mut current_language = None;
    let mut current_filename = None;
    let mut current_heading: Option<(u8, Vec<Event>)> = None;

    let mut headings = Vec::new();
    let mut current_highlighting: (HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
        (HashSet::new(), HashSet::new(), HashSet::new());

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
                        match highlighter.lock().unwrap().highlight_to_string(inkjet_lang, &formatter::Html, &code_content) {
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
                            "<span{line_class}><span class=\"line-number\">{:0width$}</span><span class=\"code-line\">{}</span></span>", 
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
                        r#"<div class="code-block">
                            <div class="code-header">
                                <span class="code-filename">{}</span>
                                <button class="copy-button" onclick="copyCode(this)">Copy</button>
                            </div>
                            <pre><code>{}</code></pre>
                        </div>"#,
                        filename,
                        line_numbered_html
                    )
                } else {
                    format!(
                        r#"<div class="code-block">
                            <div class="code-header">
                                <button class="copy-button" onclick="copyCode(this)">Copy</button>
                            </div>
                            <pre><code>{}</code></pre>
                        </div>"#,
                        line_numbered_html
                    )
                };
                
                events.push(Event::Html(code_html.into()));
                
                current_language = None;
                current_filename = None;
                current_highlighting = (HashSet::new(), HashSet::new(), HashSet::new());
            }
            Event::Start(Tag::Heading { level, .. }) => {
                current_heading = match level {
                    pulldown_cmark::HeadingLevel::H1 => Some((1, Vec::new())),
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

                    headings.push((level, text_content.clone(), slug.clone()));
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

    (html_output, headings)
}

struct MarkdownCache {
    entries: HashMap<String, (String, Vec<(u8, String, String)>)>,
}

impl MarkdownCache {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn get(&self, path: &str) -> Option<(String, Vec<(u8, String, String)>)> {
        self.entries.get(path).map(|(html, headings)| (html.clone(), headings.clone()))
    }

    fn set(&mut self, path: String, html: String, headings: Vec<(u8, String, String)>) {
        self.entries.insert(path, (html, headings));
    }
}

lazy_static! {
    static ref MARKDOWN_CACHE: Mutex<MarkdownCache> = Mutex::new(MarkdownCache::new());
    static ref FRONTMATTER_REGEX: Regex = Regex::new(r"(?s)^-{3,}\s*\n(.*?)\n-{3,}\s*\n(.*)").unwrap();
}

async fn view_markdown(
    app_state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    let path_param = &path.0;
    let base_path = PathBuf::from("content");
    let mut file_path = base_path.join(path_param);

    if !file_path.is_file() {
        let md_path = file_path.with_extension("md");
        if md_path.is_file() {
            file_path = md_path;
        } else if file_path.is_dir() {
            let index_path = file_path.join("index.md");
            if index_path.is_file() {
                file_path = index_path;
            } else {
                let context = tera::Context::new();
                let html = app_state.tera
                    .render("404.html", &context)
                    .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
                return Ok(HttpResponse::NotFound()
                    .content_type("text/html")
                    .body(html));
            }
        } else {
            let context = tera::Context::new();
            let html = app_state.tera
                .render("404.html", &context)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
            return Ok(HttpResponse::NotFound()
                .content_type("text/html")
                .body(html));
        }
    }

    let cache_key = file_path.to_string_lossy().to_string();
    let cached_content = {
        let cache = MARKDOWN_CACHE.lock().unwrap();
        cache.get(&cache_key)
    };

    let (content_html, headings, frontmatter) = if let Some((html, headings)) = cached_content {
        let raw_content = fs::read_to_string(&file_path)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Could not read file"))?;
        
        let frontmatter = if let Some(caps) = FRONTMATTER_REGEX.captures(&raw_content) {
            let yaml_str = caps.get(1).unwrap().as_str();
            serde_yaml::from_str(yaml_str).unwrap_or_else(|e| {
                eprintln!("Frontmatter parse error in {}: {}", path_param, e);
                JsonValue::Null
            })
        } else {
            eprintln!("No frontmatter found in {}", path_param);
            JsonValue::Null
        };
        
        (html, headings, frontmatter)
    } else {
        let raw_content = fs::read_to_string(&file_path)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Could not read file"))?;
        
        let (frontmatter, body) = if let Some(caps) = FRONTMATTER_REGEX.captures(&raw_content) {
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

        let (content_html, headings) = markdown_to_html(body, &app_state.highlighter);
        
        {
            let mut cache = MARKDOWN_CACHE.lock().unwrap();
            cache.set(cache_key, content_html.clone(), headings.clone());
        }
        
        (content_html, headings, frontmatter)
    };

    let processed_frontmatter = if let JsonValue::Object(mut map) = frontmatter {
        if !map.contains_key("title") {
            eprintln!("Missing title in frontmatter for {}", path_param);
            map.insert("title".to_string(), JsonValue::String("Untitled".to_string()));
        }
        JsonValue::Object(map)
    } else {
        JsonValue::Object({
            let mut map = serde_json::Map::new();
            map.insert("title".to_string(), JsonValue::String("Untitled".to_string()));
            map
        })
    };

    let mut context = tera::Context::new();
    
    if let JsonValue::Object(fm_map) = processed_frontmatter {
        for (key, value) in fm_map {
            context.insert(key, &value);
        }
    }

    let file_tree = get_file_tree(&app_state.file_tree);
    
    context.insert("headings", &headings);
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

async fn index(
    app_state: web::Data<AppState>,
    _: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let file_tree = get_file_tree(&app_state.file_tree);

    let mut context = tera::Context::new();
    context.insert("file_tree", &file_tree);

    let html = app_state.tera
        .render("index.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

async fn resume() -> impl Responder {
    let pdf_path = "./static/pdfs/resume.pdf";
    
    match fs::read(pdf_path) {
        Ok(content) => {
            HttpResponse::Ok()
                .content_type("application/pdf")
                .append_header(("Content-Disposition", "inline"))
                .body(content)
        },
        Err(_) => {
            HttpResponse::NotFound().body("PDF not found")
        }
    }
}

async fn generate_og_image(
    app_state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    let path_param = &path.0;
    let base_path = PathBuf::from("content");
    let mut file_path = base_path.join(path_param);

    if !file_path.is_file() {
        let md_path = file_path.with_extension("md");
        if md_path.is_file() {
            file_path = md_path;
        } else if file_path.is_dir() {
            let index_path = file_path.join("index.md");
            if index_path.is_file() {
                file_path = index_path;
            } else {
                return Ok(HttpResponse::NotFound().body("Content not found"));
            }
        } else {
            return Ok(HttpResponse::NotFound().body("Content not found"));
        }
    }

    let raw_content = fs::read_to_string(&file_path)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not read file"))?;
    
    let title = if let Some(caps) = FRONTMATTER_REGEX.captures(&raw_content) {
        let yaml_str = caps.get(1).unwrap().as_str();
        if let Ok(yaml) = serde_yaml::from_str::<JsonValue>(yaml_str) {
            if let Some(title) = yaml.get("title") {
                if let Some(title_str) = title.as_str() {
                    title_str.to_string()
                } else {
                    file_path.file_stem().unwrap_or_default().to_string_lossy().to_string()
                }
            } else {
                file_path.file_stem().unwrap_or_default().to_string_lossy().to_string()
            }
        } else {
            file_path.file_stem().unwrap_or_default().to_string_lossy().to_string()
        }
    } else {
        file_path.file_stem().unwrap_or_default().to_string_lossy().to_string()
    };

    let dir_path = file_path
        .parent()
        .and_then(|p| p.strip_prefix(&base_path).ok())
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| String::from("/"));
    
    let current_dir = std::env::current_dir()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not get current directory"))?;
    
    let bg_image_path = if path_param.starts_with("notes/") || dir_path.starts_with("notes") {
        current_dir.join("static/_priv/og/notes.png")
    } else if path_param.starts_with("blog/") || dir_path.starts_with("blog") {
        current_dir.join("static/_priv/og/blog.png")
    } else if path_param.starts_with("poems/") || dir_path.starts_with("poems") {
        current_dir.join("static/_priv/og/poems.png")
    } else {
        current_dir.join("static/_priv/og/notes.png")
    };

    let mut img = if let Ok(bg_img) = image::open(&bg_image_path) {
        let resized_bg = bg_img.resize_to_fill(1200, 630, image::imageops::FilterType::Lanczos3);
        resized_bg.to_rgba8()
    } else {
        let mut fallback = ImageBuffer::new(1200, 630);
        for pixel in fallback.pixels_mut() {
            *pixel = Rgba([40, 40, 40, 255]);
        }
        fallback
    };

    let title_font = &*app_state.title_font;
    let path_font = &*app_state.path_font;

    let text_color = Rgba([255, 255, 255, 255]);
    
    let title_scale = if title.len() > 30 {
        PxScale { x: 72.0, y: 72.0 }
    } else if title.len() > 20 {
        PxScale { x: 86.0, y: 86.0 }
    } else {
        PxScale { x: 96.0, y: 96.0 } 
    };

    drawing::draw_text_mut(
        &mut img,
        text_color,
        100, 
        200, 
        title_scale,
        title_font,
        &title,
    );
    
    let path_scale = PxScale { x: 36.0, y: 36.0 };
    let path_text = format!("/{}", dir_path);
    
    drawing::draw_text_mut(
        &mut img,
        Rgba([240, 240, 240, 255]), 
        100,
        500,
        path_scale,
        path_font,
        &path_text,
    );
    
    let avatar_size = 50;
    let avatar_x = 1200 - avatar_size - 30;
    let avatar_y = 630 - avatar_size - 30;
    
    let avatar_lock = app_state.avatar.read().await;
    if let Some(avatar_img) = &*avatar_lock {
        let resized_avatar = avatar_img.resize_exact(
            avatar_size as u32, 
            avatar_size as u32, 
            image::imageops::FilterType::Lanczos3
        );
        let avatar_rgba = resized_avatar.to_rgba8();
        
        for y in 0..avatar_size {
            for x in 0..avatar_size {
                let center_x = avatar_size as f32 / 2.0;
                let center_y = avatar_size as f32 / 2.0;
                let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                
                if distance <= center_x && (avatar_x + x) < 1200 && (avatar_y + y) < 630 {
                    let avatar_pixel = avatar_rgba.get_pixel(x as u32, y as u32);
                    img.put_pixel(
                        (avatar_x + x) as u32, 
                        (avatar_y + y) as u32, 
                        *avatar_pixel
                    );
                }
            }
        }
    }

    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            &dynamic_img.to_rgba8().into_raw(),
            dynamic_img.width(),
            dynamic_img.height(),
            image::ExtendedColorType::Rgba8,
        )
        .expect("Failed to encode image");
        
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(bytes))
}

async fn generate_web_og_image(
    app_state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    let path_segment = &path.0;
    
    let (title, subtitle) = match path_segment.as_str() {
        "index" => ("namishh", "personal website and garden"),
        "about" => ("namishh", "learn more about me"),
        "stuff" => ("namishh", "stuff i have built"),
        _ => return Ok(HttpResponse::NotFound().body("Invalid web path"))
    };
    
    let current_dir = std::env::current_dir()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not get current directory"))?;
    
    let bg_image_path = current_dir.join("static/_priv/og/others.png");
    
    let mut img = if let Ok(bg_img) = image::open(&bg_image_path) {
        let resized_bg = bg_img.resize_to_fill(1200, 630, image::imageops::FilterType::Lanczos3);
        resized_bg.to_rgba8()
    } else {
        let mut fallback = ImageBuffer::new(1200, 630);
        for (x, _, pixel) in fallback.enumerate_pixels_mut() {
            let gradient = (x as f32 / 1200.0 * 30.0) as u8;
            *pixel = Rgba([30 + gradient, 30 + gradient, 50 + gradient, 255]);
        }
        fallback
    };

    let title_font = &*app_state.title_font;
    let path_font = &*app_state.path_font;

    let title_scale = PxScale { x: 120.0, y: 120.0 };
    drawing::draw_text_mut(
        &mut img,
        Rgba([255, 255, 255, 255]),
        100,
        200,
        title_scale,
        title_font,
        title,
    );
    
    let subtitle_scale = PxScale { x: 48.0, y: 48.0 };
    drawing::draw_text_mut(
        &mut img,
        Rgba([240, 240, 240, 255]),
        100,
        320,
        subtitle_scale,
        path_font,
        subtitle,
    );
    
    let avatar_size = 150;
    let avatar_x = 1200 - avatar_size - 80;
    let avatar_y = 80;
    
    let avatar_lock = app_state.avatar.read().await;
    if let Some(avatar_img) = &*avatar_lock {
        let resized_avatar = avatar_img.resize_exact(
            avatar_size as u32, 
            avatar_size as u32, 
            image::imageops::FilterType::Lanczos3
        );
        let avatar_rgba = resized_avatar.to_rgba8();
        
        for y in 0..avatar_size {
            for x in 0..avatar_size {
                let center_x = avatar_size as f32 / 2.0;
                let center_y = avatar_size as f32 / 2.0;
                let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                
                if distance <= center_x && (avatar_x + x) < 1200 && (avatar_y + y) < 630 {
                    let avatar_pixel = avatar_rgba.get_pixel(x as u32, y as u32);
                    img.put_pixel(
                        (avatar_x + x) as u32, 
                        (avatar_y + y) as u32, 
                        *avatar_pixel
                    );
                }
            }
        }
    }

    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            &dynamic_img.to_rgba8().into_raw(),
            dynamic_img.width(),
            dynamic_img.height(),
            image::ExtendedColorType::Rgba8,
        )
        .expect("Failed to encode image");
        
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(bytes))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let highlighter = Arc::new(Mutex::new(Highlighter::new()));
    let base_path = Path::new("content");
    
    let initial_tree = build_file_tree(base_path, Path::new(""));
    let file_tree = Arc::new(initial_tree);

    let title_font_data: &'static [u8] = include_bytes!("../static/_priv/fonts/Outfit-ExtraBold.ttf");
    let title_font = FontRef::try_from_slice(title_font_data).expect("Error loading title font");
    let title_font_arc = Arc::new(title_font);

    let path_font_data: &'static [u8] = include_bytes!("../static/_priv/fonts/Outfit-Medium.ttf");
    let path_font = FontRef::try_from_slice(path_font_data).expect("Error loading path font");
    let path_font_arc = Arc::new(path_font);

    let avatar = Arc::new(tokio::sync::RwLock::new(None));
    let avatar_for_closure = avatar.clone();

    let mut address = "127.0.0.1:8080";
    if let Ok(arg) = env::var("ENVIRONMENT") {
        if arg == "PRODUCTION" {
            address = "0.0.0.0:8080";
        }
    }

    let server = HttpServer::new(move || {
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
            file_tree: file_tree.clone(),
            title_font: title_font_arc.clone(),
            path_font: path_font_arc.clone(),
            avatar: avatar_for_closure.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/resume").route(web::get().to(resume)))
            .service(web::resource("/og/content/{path:.*}").route(web::get().to(generate_og_image)))
            .service(web::resource("/og/web/{path:.*}").route(web::get().to(generate_web_og_image)))
            .service(web::resource("/{path:.*}").route(web::get().to(view_markdown)))
    })
    .bind(address)?
    .run();

    actix_rt::spawn(async move {
        let fetch_avatar = || async {
            if let Ok(response) = reqwest::get("https://github.com/namishh.png").await {
                if response.status().is_success() {
                    if let Ok(bytes) = response.bytes().await {
                        if let Ok(img) = image::load_from_memory(&bytes) {
                            return Some(img);
                        }
                    }
                }
            }
            None
        };

        if let Some(img) = fetch_avatar().await {
            let mut avatar_lock = avatar.write().await;
            *avatar_lock = Some(img);
        }

        let mut interval = time::interval(Duration::from_secs(600));
        loop {
            interval.tick().await;
            if let Some(img) = fetch_avatar().await {
                let mut avatar_lock = avatar.write().await;
                *avatar_lock = Some(img);
            }
        }
    });

    server.await
}