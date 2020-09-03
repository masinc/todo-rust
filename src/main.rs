use actix_web::{get, App, HttpResponse, HttpServer};
use askama::Template;
use thiserror::Error;

struct TodoEntry {
    id: usize,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

impl actix_web::ResponseError for AppError {}

type AppResult<T> = Result<T, AppError>;

#[get("/")]
async fn index() -> AppResult<HttpResponse> {
    let mut entries = vec![];
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });

    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });

    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
