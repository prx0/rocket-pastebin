pub use crate::pastebin::paste_id::PasteId;

use crate::rocket;

use rocket::fs::NamedFile;
use rocket::response::{status};
use rocket::data::{Data, ToByteUnit};
use rocket::tokio::fs;
use std::env;

const HTTP_RESOURCE: &str = "/pastebin";

#[get("/")]
pub fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
pub async fn upload(paste: Data<'_>) -> Result<String, std::io::Error> {
    let id = PasteId::new();
    let filename = format!("upload/{id}", id = id);

    let host = env::var("ROCKET_HOST").expect("ROCKET_HOST must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    let file_upload_limit = env::var("FILE_UPLOAD_LIMIT").expect("FILE_UPLOAD_LIMIT must be set").parse::<i32>().unwrap();

    let url = format!("{host}:{port}{resource}/{id}\n", host = host, port = port, resource = HTTP_RESOURCE, id = id);
    
    paste.open(file_upload_limit.kibibytes()).into_file(filename).await?;
    Ok(url)
}

#[get("/<id>")]
pub async fn get_by_id(id: PasteId<'_>) -> Option<NamedFile> {
    let filename = format!("upload/{id}", id = id);
    NamedFile::open(&filename).await.ok()
}

#[delete("/<id>")]
pub async fn delete_by_id(id: PasteId<'_>) -> Result<String, status::NotFound<String>> {
    let file_path = format!("upload/{id}", id = id);

    match fs::remove_file(file_path).await {
        Ok(_) => Ok(format!("pastebin {} deleted", id)),
        Err(_) => Err(status::NotFound(format!("pastebin {} not found", id)))
    }
}

#[put("/<id>", data = "<paste>")]
pub async fn update_by_id(id: PasteId<'_>, paste: Data<'_>) -> Result<String, status::NotFound<String>> {
    match NamedFile::open(format!("upload/{}", id)).await {
        Ok(file) => {
            let file_upload_limit = env::var("FILE_UPLOAD_LIMIT").expect("FILE_UPLOAD_LIMIT must be set").parse::<i32>().unwrap();
            paste.open(file_upload_limit.kibibytes()).into_file(file.path()).await.unwrap();
            Ok(format!("pastebin {} updated", id))
        },
        Err(e) => Err(status::NotFound(format!("pastebin {}", e)))
    }
}

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