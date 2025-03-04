use tera::Tera;

pub fn init_tera() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(t) => {
            t
        },
        Err(e) => {
            eprintln!("Template parsing error(s): {}", e);
            std::process::exit(1);
        }
    }
}