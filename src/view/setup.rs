use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TITLE: String = ui_index_title();
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("src/view/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

fn ui_index_title() -> String {
    let title: String =
        "<h1>This is the Title 2</h1><h2>This is a smaller Title 2</h2><p>This is a paragraaf 2</p>"
            .to_owned();
    return title;
}
