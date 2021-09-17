use actix_web::{App, HttpResponse, HttpServer, ResponseError, get};
use thiserror::Error;
use askama::Template;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>
}

let mut entries = Vec::new();
entries.push(TodoEntry {
    id: 1,
    text: "first entry".to_string(),
});
entries.push(TodoEntry {
    id: 2,
    text: "second entry".to_string(),
});

#[derive(Error, Debug)]
enum MyError {}
impl ResponseError for MyError {
    
}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world";

    Ok(HttpResponse::Ok().body(response_body))

}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
