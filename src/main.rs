use actix_web::{get, web, App, HttpResponse, HttpServer};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use thiserror::Error;

struct TodoEntry {
    id: u32,
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

    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl actix_web::ResponseError for AppError {}

type AppResult<T> = Result<T, AppError>;

type WebDataBasePool = web::Data<Pool<SqliteConnectionManager>>;

#[get("/")]
async fn index(db: WebDataBasePool) -> AppResult<HttpResponse> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;

    let rows = statement.query_map(rusqlite::params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;

        Ok(TodoEntry { id, text })
    })?;

    let mut entries = vec![];
    for row in rows {
        entries.push(row?);
    }

    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

const SQL_CREATE_TABLE: &str = "\
CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    text TEXT NOT NULL
)";

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool");
    conn.execute(SQL_CREATE_TABLE, rusqlite::params![])
        .expect("Failed to create a table `todo`.");
    HttpServer::new(move || App::new().service(index).data(pool.clone()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
