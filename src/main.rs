#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;

pub mod pastebin;

use dotenv::dotenv;
use rocket::fs::NamedFile;
use std::thread;
use std::time::Duration;
use rocket_dyn_templates::Template;


// Needed stuff for tera templating engine
// in order to generate web page

#[get("/style.css")]
async fn style() -> Option<NamedFile> {
    NamedFile::open("templates/css/base.css").await.ok()
}

#[get("/prism.css")]
async fn prism_css() -> Option<NamedFile> {
    NamedFile::open("templates/css/prism.css").await.ok()
}

#[get("/prism.js")]
async fn prism_js() -> Option<NamedFile> {
    NamedFile::open("templates/js/prism.js").await.ok()
}

#[launch]
fn rocket() -> _ {
    use pastebin::routes;
    dotenv().ok();

    // Clean old pastebins
    thread::spawn(|| {
        loop {
            pastebin::clean_old_pastes();
            thread::sleep(Duration::from_secs(1));
        }
    });

    rocket::build()
        .mount("/", routes![
            style,
            prism_css,
            prism_js,
            routes::show_pastebin,
            routes::index
        ])
        .mount("/api/pastebin", routes![
            routes::upload,
            routes::upload_form,
            routes::get_by_id,
            routes::delete_by_id,
            routes::update_by_id,
        ])
        .attach(Template::fairing())
}