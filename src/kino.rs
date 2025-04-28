use actix_web::{web, HttpResponse, Result};
use crate::state::AppState;
use crate::file_tree::get_file_tree;
use tera::Context;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
struct YouTubeChannel {
    name: String,
    url: String,
    description: String,
}

#[derive(Serialize)]
struct Blog {
    name: String,
    url: String,
    description: String,
}

#[derive(Serialize)]
struct TechAccount {
    name: String,
    username: String,
}

#[derive(Serialize)]
struct DevlogIdea {
    title: String,
    completed: bool,
}

#[derive(Serialize)]
struct KinoData {
    youtube_channels: Vec<YouTubeChannel>,
    blogs: Vec<Blog>,
    tech_accounts: Vec<TechAccount>,
    indian_tech_accounts: Vec<TechAccount>,
    devlog_ideas: Vec<DevlogIdea>,
}

pub async fn kino(
    app_state: web::Data<AppState>,
    _: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, actix_web::Error> {
    let file_tree = get_file_tree(&app_state.file_tree);
    let mut context = Context::new();
    
    let kino_data = get_kino_data();
    
    context.insert("file_tree", &file_tree);
    context.insert("kino_data", &kino_data);
    
    let html = app_state.tera
        .render("list.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok()
        .insert_header((actix_web::http::header::CACHE_CONTROL, "public, max-age=60"))
        .content_type("text/html")
        .body(html))
}

fn get_kino_data() -> KinoData {
    let youtube_channels = vec![
        YouTubeChannel {
            name: "freya holmer".to_string(),
            url: "https://www.youtube.com/@acegikmo".to_string(),
            description: "beautiful videos on game dev and the mathematics behind game dev".to_string(),
        },
        YouTubeChannel {
            name: "colin galen".to_string(),
            url: "https://www.youtube.com/@ColinGalen/".to_string(),
            description: "competitive programming tips and tricks".to_string(),
        },
        YouTubeChannel {
            name: "argonautcode".to_string(),
            url: "https://www.youtube.com/@argonautcode/".to_string(),
            description: "".to_string(),
        },
        YouTubeChannel {
            name: "fractal philosphy".to_string(),
            url: "https://www.youtube.com/@FractalPhilosophy/".to_string(),
            description: "".to_string(),
        },
        YouTubeChannel {
            name: "sphaerophoria".to_string(),
            url: "https://www.youtube.com/@sphaerophoria".to_string(),
            description: "programming livestreams with zig".to_string(),
        },
    ];

    let blogs = vec![
        Blog {
            name: "veysel".to_string(),
            url: "https://veysel.bearblog.dev/".to_string(),
            description: "excellent curation of resources for linguistics and plt".to_string(),
        },
        Blog {
            name: "kennethnym".to_string(),
            url: "https://kennethnym.com/".to_string(),
            description: "well written articles on programming habits".to_string(),
        },
        Blog {
            name: "snats".to_string(),
            url: "https://snats.xyz/pages/articles.html".to_string(),
            description: "".to_string(),
        },
        Blog {
            name: "ludwig".to_string(),
            url: "https://ludwigabap.bearblog.dev/".to_string(),
            description: "great resources for ml and cs".to_string(),
        },
        Blog {
            name: "mcyoung".to_string(),
            url: "https://mcyoung.xyz/posts".to_string(),
            description: "compilers and performance".to_string(),
        },
        Blog {
            name: "maharshi".to_string(),
            url: "https://maharshi.bearblog.dev/blog/".to_string(),
            description: "cuda and ml".to_string(),
        },
    ];

    let tech_accounts = vec![
        TechAccount {
            name: "seatedro".to_string(),
            username: "seatedro".to_string(),
        },
        TechAccount {
            name: "ludwig".to_string(),
            username: "ludwigABAP".to_string(),
        },
        TechAccount {
            name: "zoe".to_string(),
            username: "zoriya_dev".to_string(),
        },
        TechAccount {
            name: "kenneth".to_string(),
            username: "kennethnym".to_string(),
        },
        TechAccount {
            name: "vin".to_string(),
            username: "vin_acct".to_string(),
        },
        TechAccount {
            name: "aryas".to_string(),
            username: "Aryvyo".to_string(),
        },
        TechAccount {
            name: "aster".to_string(),
            username: "4ster_light".to_string(),
        },
        TechAccount {
            name: "marin".to_string(),
            username: "marinn1_".to_string(),
        },
        TechAccount {
            name: "char".to_string(),
            username: "cunjur".to_string(),
        },
    ];

    let indian_tech_accounts = vec![
        TechAccount {
            name: "rex".to_string(),
            username: "rexmkv".to_string(),
        },
        TechAccount {
            name: "rc".to_string(),
            username: "rcx86".to_string(),
        },
        TechAccount {
            name: "nsg650".to_string(),
            username: "NSG650".to_string(),
        },
        TechAccount {
            name: "cneuralnets".to_string(),
            username: "cneuralnetwork".to_string(),
        },
        TechAccount {
            name: "himanshu".to_string(),
            username: "himanshustwts".to_string(),
        },
        TechAccount {
            name: "minami".to_string(),
            username: "minamisatokun".to_string(),
        },
        TechAccount {
            name: "curlydazai".to_string(),
            username: "curlydazai".to_string(),
        },
        TechAccount {
            name: "maharshi".to_string(),
            username: "mrsiipa".to_string(),
        },
    ];

    let devlog_ideas = vec![
        DevlogIdea {
            title: "barebones game in assembly".to_string(),
            completed: false,
        },
        DevlogIdea {
            title: "shaders".to_string(),
            completed: true,
        },
        DevlogIdea {
            title: "procedural animation".to_string(),
            completed: false,
        },
        DevlogIdea {
            title: "chess engine from scratch".to_string(),
            completed: false,
        },
    ];

    KinoData {
        youtube_channels,
        blogs,
        tech_accounts,
        indian_tech_accounts,
        devlog_ideas,
    }
}