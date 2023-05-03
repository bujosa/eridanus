use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[get("/hello/{name}")]
// async fn hello(web::Path(name): web::Path<String>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Hello {}!", name))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("localhost:8080")?
        .run()
        .await
}
