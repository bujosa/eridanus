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

/// Example using Rhai
#[get("/rhai/{num1}/{num2}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();
    let mut engine = Engine::new();

    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let result: i64 = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();

    format!("{} + {} = {}", num1, num2, result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello).service(add))
        .bind("localhost:8080")?
        .run()
        .await
}
