#[macro_use] extern crate rocket;

pub mod pastebin;

use dotenv::dotenv;

#[launch]
fn rocket() -> _ {
    use pastebin::routes;

    dotenv().ok();
    rocket::build()
        .mount("/pastebin", routes![
            routes::index, 
            routes::upload, 
            routes::get_by_id,
            routes::delete_by_id,
            routes::update_by_id
        ])
}