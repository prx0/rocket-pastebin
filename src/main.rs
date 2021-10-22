#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

pub mod pastebin;

use dotenv::dotenv;
use std::thread;
use std::time::Duration;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    use pastebin::routes;
    dotenv().ok();

    thread::spawn(|| {
        loop {
            pastebin::clean_old_pastes();
            thread::sleep(Duration::from_secs(1));
        }
    });

    rocket::build()
        .mount("/pastebin", routes![
            routes::index, 
            routes::upload,
            routes::upload_form,
            routes::get_by_id,
            routes::delete_by_id,
            routes::update_by_id,
        ])
        .attach(Template::fairing())
}