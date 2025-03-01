use actix_web::{middleware, web, App, Error, HttpServer, Responder, Result};
use std::collections::HashMap;
use tera::Tera;

async fn index(
    tmpl: web::Data<tera::Tera>,
    _: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, Error> {
    let html = tmpl
        .render("index.html", &tera::Context::new())
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
    })
    .bind("0.0.0.0:8080")? 
    .run()
    .await
}
