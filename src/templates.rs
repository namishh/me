use tera::Tera;

pub fn init_tera() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(t) => {
            println!("Loaded templates: {:?}", t.get_template_names().collect::<Vec<_>>());
            t
        },
        Err(e) => {
            eprintln!("Template parsing error(s): {}", e);
            std::process::exit(1);
        }
    }
}