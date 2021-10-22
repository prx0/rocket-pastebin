pub use crate::pastebin::models::PasteId;
pub use crate::pastebin::models::Lang;
pub use crate::pastebin::models::SendPastebin;

use crate::rocket;

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::response::{status};
use rocket::data::{Data, ToByteUnit};
use rocket::tokio::fs;
use rocket::tokio::io::AsyncWriteExt;
use rocket::response::Redirect;
use rocket::http::ContentType;

use rocket_dyn_templates::{Template, tera};

use std::env;
use std::str::from_utf8;

const HTTP_RESOURCE: &str = "/api/pastebin";

// Return the home page
// used for web view 

#[get("/")]
pub fn index() -> Template {
    let mut context = tera::Context::new();
    context.insert("title", "Upload a pastebin");
    Template::render("tera/index", &context.into_json())
}

// Return the pastebin with the correct language for highlight
// used for web view

#[get("/<id>/<lang>")]
pub async fn show_pastebin(id: PasteId<'_>, lang: Lang<'_>) -> Result<Template, status::NotFound<String>> {
    let filename = format!("upload/{id}", id = &id);
    match fs::read(&filename).await {
        Ok(content) => {
            let mut context = tera::Context::new();
            context.insert("content", from_utf8(&content).unwrap());
            context.insert("title", &format!("Pastebin {}", id));
            context.insert("lang", &format!("{}", &lang));
            Ok(Template::render("tera/pastebin/show", context.into_json()))
        },
        Err(_) => Err(status::NotFound(format!("pastebin {} not found", id)))
    }
}


// Create a new pastebin
// Endpoint API

#[post("/", data = "<paste>", rank = 2)]
pub async fn upload(paste: Data<'_>) -> Result<(ContentType, String), std::io::Error> {
    let id = PasteId::new();
    let filename = format!("upload/{id}", id = id);

    let host = env::var("ROCKET_HOST").expect("ROCKET_HOST must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    let url = format!("{host}:{port}{resource}/{id}\n", host = host, port = port, resource = HTTP_RESOURCE, id = id);
    
    paste.open(128.kibibytes()).into_file(filename).await?;
    Ok((ContentType::Text, url))
}

// Create a new pastebin and redirect to show_pastebin if success
// used for web view

#[post("/", data = "<paste_form>")]
pub async fn upload_form(mut paste_form: Form<SendPastebin>) -> Result<(ContentType, Redirect), std::io::Error> {
    let id = PasteId::new();
    let filename = format!("upload/{id}", id = id);

    let mut new_file = fs::File::create(&filename).await.expect(&format!("unable to create file {}", filename));
    new_file.write_all(&paste_form.raw_content.as_bytes()).await.expect(&format!("unable to write on file {}", filename));

    Ok((ContentType::Text, Redirect::to(format!("/{}/{}", id, paste_form.lang))))
}

// Get an existing pastebin by id
// Endpoint API

#[get("/<id>")]
pub async fn get_by_id(id: PasteId<'_>) -> Option<NamedFile> {
    let filename = format!("upload/{id}", id = id);
    NamedFile::open(&filename).await.ok()
}

// Delete an existing pastebin by id
// Endpoint API

#[delete("/<id>")]
pub async fn delete_by_id(id: PasteId<'_>) -> Result<String, status::NotFound<String>> {
    let file_path = format!("upload/{id}", id = id);

    match fs::remove_file(file_path).await {
        Ok(_) => Ok(format!("pastebin {} deleted", id)),
        Err(_) => Err(status::NotFound(format!("pastebin {} not found", id)))
    }
}

// Update an existing pastebin by id
// Endpoint API
#[put("/<id>", data = "<paste>")]
pub async fn update_by_id(id: PasteId<'_>, paste: Data<'_>) -> Result<String, status::NotFound<String>> {
    match NamedFile::open(format!("upload/{}", id)).await {
        Ok(file) => {
            paste.open(128.kibibytes()).into_file(file.path()).await.unwrap();
            Ok(format!("pastebin {} updated", id))
        },
        Err(e) => Err(status::NotFound(format!("pastebin {}", e)))
    }
}

// Test for Endpoint API

#[cfg(test)]
mod test {
    use crate::pastebin::routes::HTTP_RESOURCE;

    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_upload() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let body = "Hello, world!";
        let response = client.post(HTTP_RESOURCE)
            .body(body)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_by_id() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut test_file = fs::File::create("upload/18ab80ae-c00e-4f72-b6f3-f4e2233a8b68").expect("unable to write test file");
        let test_file_content= b"Hello, world!";
        let _ = test_file.write(test_file_content);

        let response = client.get(format!("{}/{}", HTTP_RESOURCE, "18ab80ae-c00e-4f72-b6f3-f4e2233a8b68")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_bytes().unwrap(), test_file_content);
    }

    #[test]
    fn test_get_pastebin_which_not_exist() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(format!("{}/{}", HTTP_RESOURCE, "notexist")).dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_delete_by_id() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let _ = fs::File::create("upload/a8b5e8bd-3b35-4865-886a-8103c5c27909").expect("unable to write test file");
        let response = client.delete(format!("{}/{}", HTTP_RESOURCE, "a8b5e8bd-3b35-4865-886a-8103c5c27909")).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_delete_pastebin_which_not_exist() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.delete(format!("{}/test", HTTP_RESOURCE)).dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_update_by_id() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let _ = fs::File::create("upload/e62922ea-1d3a-4dc8-a5a3-52d973600bb9").expect("unable to write test file");

        let body = b"Updated Pastebin";
        let response = client.put(format!("{}/e62922ea-1d3a-4dc8-a5a3-52d973600bb9", HTTP_RESOURCE))
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_update_by_id_with_bad_id() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let body = b"Updated Pastebin";
        let response = client.put(format!("{}/test", HTTP_RESOURCE))
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }
}