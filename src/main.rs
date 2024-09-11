use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
#[get("/")]
async fn home() -> impl Responder {
    format!("Hello my dear ziiz friend!")
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(home)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}