use actix_web::{get, App, HttpResponse, HttpServer};
type ActixResult<T> = Result<T, actix_web::Error>;

#[get("/")]
async fn index() -> ActixResult<HttpResponse> {
    let response_body = "Hello World";

    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> ActixResult<()> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
