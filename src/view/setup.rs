use lazy_static::lazy_static;
use std::path::Path;
use tera::Tera;

lazy_static! {
    pub static ref TITLE: String = ui_index_title();
    pub static ref TEMPLATES: Tera = {

        let template_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("view").join("templates").join("**/*");
        let source = template_dir.to_str().expect("Invalid template directory path");

        // let source = "templates/**/*";
        let tera = Tera::new(source).expect("Couldn't find template folder");
        return tera;
    };
}

fn ui_index_title() -> String {
    let title: String =
        "<h1>This is the Title 2</h1><h2>This is a smaller Title 2</h2><p>This is a paragraaf 2</p>"
            .to_owned();
    return title;
}
