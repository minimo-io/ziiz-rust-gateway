mod api;
use actix_web::{get, web::Data, App, HttpServer, Responder, middleware::Logger};

use api::payin::pay;

// we need to get this from our settings toml files
const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 8080;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    // for debug, but still this should be retrieved from 
    // our settings toml files in the future
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    println!("Runnin on address {}:{}", SERVER_ADDR, SERVER_PORT);

    // IMPORTANT: This will be fired every time 
    // Actix-web creates a new thread to handle requests
    HttpServer::new(move || {
        let logger = Logger::default();

        App::new().wrap(logger).service(pay)
    })
    .bind((SERVER_ADDR, SERVER_PORT))?
    .run()
    .await
}