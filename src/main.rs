use actix_web::{web, App, HttpServer, middleware};
use search::initialize_search_index;
use std::sync::{Arc, Mutex};
use std::path::Path;
use ab_glyph::FontRef;
use tokio::sync::RwLock;
use std::time::Duration;
use tokio::time;
use reqwest;
use image::load_from_memory;

use crate::state::AppState;
use crate::file_tree::build_file_tree;
use crate::handlers::{index, projects, view_markdown, resume, generate_og_image, generate_web_og, generate_tweet_image, search};
use crate::templates::init_tera;
use crate::rss::rss_feed;

mod state;
mod image_generator;
mod file_tree;
mod markdown;
mod cache;
mod handlers;
mod rss;
mod templates;
mod tweet;
mod search;
mod projects;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let highlighter = Arc::new(Mutex::new(inkjet::Highlighter::new()));

    let base_path = Path::new("content");
    let initial_tree = build_file_tree(base_path, Path::new(""));
    let file_tree = Arc::new(initial_tree);

    initialize_search_index(base_path)?;

    let title_font_data: &'static [u8] = include_bytes!("../static/_priv/fonts/InterE.ttf");
    let title_font = FontRef::try_from_slice(title_font_data).expect("Error loading title font");
    let title_font_arc = Arc::new(title_font);

    let path_font_data: &'static [u8] = include_bytes!("../static/_priv/fonts/InterM.ttf");
    let path_font = FontRef::try_from_slice(path_font_data).expect("Error loading path font");
    let path_font_arc = Arc::new(path_font);

    let avatar = Arc::new(RwLock::new(None));
    let avatar_for_closure = avatar.clone();

    let mut address = "127.0.0.1:8080";
    if let Ok(arg) = std::env::var("ENVIRONMENT") {
        if arg == "PRODUCTION" {
            address = "0.0.0.0:8080";
        }
    }

    let server = HttpServer::new(move || {
        let tera = init_tera();

        let app_state = AppState {
            tera,
            highlighter: highlighter.clone(),
            file_tree: file_tree.clone(),
            title_font: title_font_arc.clone(),
            path_font: path_font_arc.clone(),
            avatar: avatar.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/static", "./static"))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/stuff").route(web::get().to(projects)))
            .service(web::resource("/resume").route(web::get().to(resume)))
            .service(web::resource("/og/content/{path:.*}").route(web::get().to(generate_og_image)))
            .service(web::resource("/og/web/{path:.*}").route(web::get().to(generate_web_og)))
            .service(web::resource("/tweet/{path:.*}").route(web::get().to(generate_tweet_image)))
            .service(web::resource("/rss.xml").route(web::get().to(rss_feed))) 
            .service(web::resource("/api/search").route(web::get().to(search))) 
            .service(web::resource("/{path:.*}").route(web::get().to(view_markdown)))
    })
    .bind(address)?
    .run();

    actix_rt::spawn(async move {
        let fetch_avatar = || async {
            if let Ok(response) = reqwest::get("https://github.com/namishh.png").await {
                if response.status().is_success() {
                    if let Ok(bytes) = response.bytes().await {
                        if let Ok(img) = load_from_memory(&bytes) {
                            return Some(img);
                        }
                    }
                }
            }
            None
        };

        if let Some(img) = fetch_avatar().await {
            let mut avatar_lock = avatar_for_closure.write().await;
            *avatar_lock = Some(img);
        }

        // update my avatar every 10 minutes
        let mut interval = time::interval(Duration::from_secs(600));
        loop {
            interval.tick().await;
            if let Some(img) = fetch_avatar().await {
                let mut avatar_lock = avatar_for_closure.write().await;
                *avatar_lock = Some(img);
            }
        }
    });

    server.await
}