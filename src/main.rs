use actix_web::{App, Error, HttpServer, Responder, Result, middleware, web};
use std::collections::HashMap;
use tera::Tera;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// Reading all the files in the content directory
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

            // Skip hidden files
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
                    path: path_str.to_string(),
                    is_dir,
                    children: Vec::new(),
                });
            }
        }
    }

    // Sort directories first, then files, both alphabetically
    nodes.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    nodes
}

async fn index(
    tmpl: web::Data<tera::Tera>,
    _: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, Error> {
    let base_path = Path::new("content");
    let file_tree = build_file_tree(base_path, Path::new(""));

    let mut context = tera::Context::new();
    context.insert("file_tree", &file_tree);

    let html = tmpl
        .render("index.html", &context)
        .expect("Failed to render template");
    Ok(web::Html::new(html))
}

async fn view_markdown(
    tmpl: web::Data<tera::Tera>,
    path: web::Path<(String,)>,
) -> Result<impl Responder, Error> {
    let file_path = PathBuf::from("content").join(&path.0);
    
    let content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(_) => "File not found or cannot be read".to_string(),
    };

    let base_path = Path::new("content");
    let file_tree = build_file_tree(base_path, Path::new(""));

    let mut context = tera::Context::new();
    context.insert("file_tree", &file_tree);
    context.insert("content", &content);
    context.insert("file_path", &path.0);

    let html = tmpl
        .render("view.html", &context)
        .expect("Failed to render template");
    Ok(web::Html::new(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        App::new()
            .app_data(web::Data::new(tera))
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/{path:.*}").route(web::get().to(view_markdown)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
