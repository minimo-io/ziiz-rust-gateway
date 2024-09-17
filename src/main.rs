mod api;
use actix_web::{get, web::Data, App, HttpServer, Responder, middleware::Logger, HttpResponse};
use sea_orm::{Database, DbConn};
use std::env;
use std::sync::Arc;
use dotenv::dotenv;

pub mod entity;

// api endpoints
use api::payin::pay;
use api::data::all_countries;
use api::home::home;


#[derive(Clone)]
struct AppState {
    db_pool: Arc<sea_orm::DatabaseConnection>,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // for debug, but still this should be retrieved from 
    // our settings toml files in the future
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();


    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in enviroment");
    let app_server_addr = env::var("APP_SERVER__ADDR").expect("APP_SERVER__ADDR must be set in enviroment");
    let app_server_port = env::var("APP_SERVER__PORT").expect("APP_SERVER__PORT must be set in enviroment");
    let app_server_port = app_server_port.parse().unwrap_or(8080); // convert to a integer or 8080

    println!("Runnin on address {}:{}", &app_server_addr, &app_server_port);

    let db: DbConn = Database::connect(&database_url).await.expect("Failed to connect to the database"); // for SeaORM
    
    
    // assert_eq!(db.ping().await.is_ok(), true, "Database ping failed!");

    if db.ping().await.is_ok() {
        println!("Database ping successful!");
    } else {
        println!("Database ping failed!");
    }

    // Create shared state
    // let app_state = Arc::new(db);

    // NOTE: This will be fired every time that Actix-web creates a new thread to handle a request
    // Shared resources must be placed above this line
    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
        .wrap(logger)
        .app_data(Data::new(db.clone())) // Store the database connection to access in services
        // .app_data(Data::new(AppState {
        //     db_pool: app_state.clone(),
        // }))        
        .service(pay)
        .service(home)
        .service(all_countries)
    })
    .bind((app_server_addr.as_str(), app_server_port))?
    .run()
    .await
}

// CLOSE CONNECTION WHEN EXIT ???
// let db = Database::connect(url).await?;
// Closing connection here
// db.close().await?;