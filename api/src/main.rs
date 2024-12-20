use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, DbConn};
use std::env;
use std::sync::Arc;

use sea_orm::{DbErr, EntityTrait, ModelTrait, QueryFilter, Set};

use std::time::Duration;

use ziiz::utils::date_utils::format_date;

pub mod entities;
mod routes; // export it for use among the app

// api endpoints
use routes::data::all_countries;
use routes::home::home;
use routes::payin::pay;

// #[derive(Clone)]
// struct AppState {
//     db_pool: Arc<sea_orm::DatabaseConnection>,
// }

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // for debug, but still this should be retrieved from
    // our settings toml files in the future
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "0");
    env_logger::init();

    // this enviroment handling should go to the config mod
    // once that is done delete this
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in enviroment");
    let app_server_addr =
        env::var("APP_SERVER__ADDR").expect("APP_SERVER__ADDR must be set in enviroment");
    let app_server_port =
        env::var("APP_SERVER__PORT").expect("APP_SERVER__PORT must be set in enviroment");
    let app_server_port = app_server_port.parse().unwrap_or(8080); // convert to a integer or 8080

    println!(
        "Runnin on address {}:{}",
        &app_server_addr, &app_server_port
    );

    // let db: DbConn = Database::connect(&database_url)
    //     .await
    //     .expect("Failed to connect to the database"); // for SeaORM

    // let connect_options = ConnectOptions::new(database_url)
    //     .max_connections(100)
    //     .min_connections(5)
    //     .connect_timeout(Duration::from_secs(30))
    //     .acquire_timeout(Duration::from_secs(30))
    //     .idle_timeout(Duration::from_secs(60));

    // Create a connection pool

    let db: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // let db = Arc::new(db);

    // assert_eq!(db.ping().await.is_ok(), true, "Database ping failed!");

    // if db.ping().await.is_ok() {
    //     println!("Database ping successful!");
    // } else {
    //     println!("Database ping failed!");
    // }

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
    // .workers(10)
    .bind((app_server_addr.as_str(), app_server_port))?
    .run()
    .await
}

// CLOSE CONNECTION WHEN FORCE EXIT (SeaORM quits db on drop but...)
// let db = Database::connect(url).await?;
// Closing connection here
// db.close().await?;
