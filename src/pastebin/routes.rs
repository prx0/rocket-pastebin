pub use crate::pastebin::paste_id::PasteId;

use crate::rocket;
use rocket::data::{Data, ToByteUnit};
use rocket::response::Debug;
use rocket::tokio::fs::File;

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
pub async fn upload(paste: Data<'_>) -> Result<String, Debug::<std::io::Error>> {
    let id = PasteId::new(10);
    let filename = format!("upload/{id}", id = id);
    
    let host = env::var("ROCKET_HOST").expect("HOST must be set");
    let port = env::var("ROCKET_PORT").expect("PORT must be set");

    let url = format!("{host}:{port}{resource}/{id}\n", host = host, port = port, resource = HTTP_RESOURCE, id = id);
    
    paste.open(128.kibibytes()).into_file(filename).await?;
    Ok(url)
}

#[get("/<id>")]
pub async fn get_by_id(id: PasteId<'_>) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).await.ok()
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
        let mut test_file = fs::File::create("upload/testfile").expect("unable to write test file");
        let test_file_content= b"Hello, world!";
        let _ = test_file.write(test_file_content);

        let response = client.get(format!("{}/{}", HTTP_RESOURCE, "testfile")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_bytes().unwrap(), test_file_content);
    }
}