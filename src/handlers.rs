use actix_web::{web, HttpResponse, Result, Responder};
use crate::state::AppState;
use crate::file_tree::get_file_tree;
use crate::markdown::{markdown_to_html, extract_frontmatter};
use crate::cache::MARKDOWN_CACHE;
use std::fs;
use std::path::PathBuf;
use tera::Context;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use crate::image_generator::{generate_content_og_image, generate_web_og_image};
use crate::tweet::generate_tweet;
use serde::Deserialize;
use crate::search::search_content;
use crate::projects::get_projects;

pub async fn index(
    app_state: web::Data<AppState>,
    _: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, actix_web::Error> {
    let file_tree = get_file_tree(&app_state.file_tree);
    let mut context = Context::new();
    context.insert("file_tree", &file_tree);
    let html = app_state.tera
        .render("index.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn projects(
    app_state: web::Data<AppState>,
    _: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, actix_web::Error> {
    let file_tree = get_file_tree(&app_state.file_tree);
    let mut context = Context::new();
    context.insert("file_tree", &file_tree);
    context.insert("projects", &get_projects());
    let html = app_state.tera
        .render("projects.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .insert_header((actix_web::http::header::CACHE_CONTROL, "public, max-age=60"))
        .content_type("text/html")
        .body(html))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    #[serde(default)]
    q: String,
}


pub async fn search_page(
    app_state: web::Data<AppState>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let file_tree = get_file_tree(&app_state.file_tree);
    let mut context = Context::new();
    
    context.insert("file_tree", &file_tree);
    
    if !query.q.is_empty() {
        let results = search_content(&query.q);
        context.insert("results", &results);
        context.insert("query", &query.q);
        context.insert("has_query", &true);
    } else {
        context.insert("has_query", &false);
    }
    
    let html = app_state.tera
        .render("search.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok()
        .insert_header((actix_web::http::header::CACHE_CONTROL, "public, max-age=60"))
        .content_type("text/html")
        .body(html))
}

pub async fn view_markdown(
    app_state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, actix_web::Error> {
    let path_param = &path.0;
    let base_path = PathBuf::from("content");
    let mut file_path = base_path.join(path_param);

    let file_tree = get_file_tree(&app_state.file_tree);

    if !file_path.is_file() {
        let md_path = file_path.with_extension("md");
        if md_path.is_file() {
            file_path = md_path;
        } else if file_path.is_dir() {
            let index_path = file_path.join("index.md");
            if index_path.is_file() {
                file_path = index_path;
            } else {
                let mut context = Context::new();
                context.insert("file_tree", &file_tree);
                let html = app_state.tera
                    .render("404.html", &context)
                    .map_err(|e| {
                        eprintln!("Tera error: {:?}", e);
                        actix_web::error::ErrorInternalServerError("Template rendering failed")
                    })?;
                return Ok(HttpResponse::NotFound().content_type("text/html").body(html));
            }
        } else {
            let mut context = Context::new();
            context.insert("file_tree", &file_tree);
            let html = app_state.tera
                .render("404.html", &context)
                .map_err(|e| {
                    eprintln!("Tera error: {:?}", e);
                    actix_web::error::ErrorInternalServerError("Template rendering failed")
                })?;
            return Ok(HttpResponse::NotFound().content_type("text/html").body(html));
        }
    }

    let cache_key = file_path.to_string_lossy().to_string();
    let current_modified = fs::metadata(&file_path)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not get file metadata"))?
        .modified()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not get last modified time"))?;
    let mut cache = MARKDOWN_CACHE.lock().unwrap();
    let (content_html, headings, frontmatter) = if let Some((html, headings)) = cache.get_if_fresh(&cache_key, current_modified) {
        let raw_content = fs::read_to_string(&file_path)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Could not read file"))?;
        let (frontmatter, _) = extract_frontmatter(&raw_content);
        (html, headings, frontmatter)
    } else {
        let raw_content = fs::read_to_string(&file_path)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Could not read file"))?;
        let (frontmatter, body) = extract_frontmatter(&raw_content);
        let (content_html, headings) = markdown_to_html(body, &app_state.highlighter);
        cache.set(cache_key.clone(), current_modified, content_html.clone(), headings.clone());
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

    let mut context = Context::new();
    if let JsonValue::Object(fm_map) = processed_frontmatter {
        for (key, value) in fm_map {
            context.insert(key, &value);
        }
    }
    context.insert("headings", &headings);
    context.insert("file_tree", &file_tree);
    context.insert("content", &content_html);
    context.insert("file_path", &path_param);

    let html = app_state.tera
        .render("view.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    let last_modified_header = actix_web::http::header::LastModified(current_modified.into());
    Ok(HttpResponse::Ok()
        .insert_header((actix_web::http::header::CACHE_CONTROL, "public, max-age=0"))
        .insert_header((actix_web::http::header::LAST_MODIFIED, last_modified_header))
        .content_type("text/html")
        .body(html))
}

pub async fn generate_tweet_image( 
    app_state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, actix_web::Error> {
    let title_font = &*app_state.title_font;
    let path_font: &ab_glyph::FontRef<'_> = &*app_state.path_font;
    let id = &path.0;

    let image_bytes = generate_tweet(id, title_font, path_font).await.expect("Failed to generate tweet image");

    // Return the HTTP response
    Ok(HttpResponse::Ok().content_type("image/png").body(image_bytes))
}

pub async fn resume() -> impl Responder {
    let pdf_path = "./static/pdfs/resume.pdf";
    match fs::read(pdf_path) {
        Ok(content) => HttpResponse::Ok()
            .content_type("application/pdf")
            .append_header(("Content-Disposition", "inline"))
            .body(content),
        Err(_) => HttpResponse::NotFound().body("PDF not found"),
    }
}

pub async fn search(
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let search_term = &query.q;
    
    let results = search_content(search_term);
    
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(results))
}



pub async fn generate_og_image(
    app_state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, actix_web::Error> {
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

    let raw_content = std::fs::read_to_string(&file_path)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not read file"))?;
    let title = if let Some(caps) = crate::markdown::FRONTMATTER_REGEX.captures(&raw_content) {
        let yaml_str = caps.get(1).unwrap().as_str();
        if let Ok(yaml) = serde_yaml::from_str::<JsonValue>(yaml_str) {
            if let Some(title) = yaml.get("title") {
                title.as_str().unwrap_or(&file_path.file_stem().unwrap_or_default().to_string_lossy()).to_string()
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

    let title_font = &*app_state.title_font;
    let path_font = &*app_state.path_font;
    let avatar_lock = app_state.avatar.read().await;
    let avatar = avatar_lock.as_ref().cloned();

    let image_bytes = generate_content_og_image(&title, &dir_path, title_font, path_font, &avatar);

    Ok(HttpResponse::Ok()
        .insert_header((actix_web::http::header::CACHE_CONTROL, "public, max-age=3600"))
        .content_type("image/png")
        .body(image_bytes))
}

pub async fn generate_web_og(
    app_state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, actix_web::Error> {
    let path_segment = &path.0;
    let (title, subtitle) = match path_segment.as_str() {
        "index" => ("namishh", "personal website and garden"),
        "about" => ("namishh", "learn more about me"),
        "stuff" => ("namishh", "stuff i have built"),
        _ => return Ok(HttpResponse::NotFound().body("Invalid web path")),
    };

    let title_font = &*app_state.title_font;
    let path_font = &*app_state.path_font;
    let avatar_lock = app_state.avatar.read().await;
    let avatar = avatar_lock.as_ref().cloned();

    let image_bytes = generate_web_og_image(title, subtitle, title_font, path_font, &avatar);

    Ok(HttpResponse::Ok()
        .insert_header((actix_web::http::header::CACHE_CONTROL, "public, max-age=3600"))
        .content_type("image/png")
        .body(image_bytes))
}