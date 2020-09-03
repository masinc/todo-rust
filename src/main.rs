use actix_web::{get, App, HttpResponse, HttpServer};
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {}

impl actix_web::ResponseError for AppError {}

type AppResult<T> = Result<T, AppError>;

#[get("/")]
async fn index() -> AppResult<HttpResponse> {
    let response_body = "Hello World";

    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> AppResult<()> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
