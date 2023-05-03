use actix_web::{get, web::Path, App, HttpResponse, HttpServer, Responder};

use rhai::Engine;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// Example of a dynamic path.
#[get("/hello/{name}")]
async fn hello(name: Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind("localhost:8080")?
        .run()
        .await
}
