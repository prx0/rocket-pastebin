#[macro_use] extern crate rocket;

pub mod pastebin;

use dotenv::dotenv;
use std::thread;
use std::time::Duration;

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
            routes::get_by_id,
            routes::delete_by_id,
            routes::update_by_id
        ])
}