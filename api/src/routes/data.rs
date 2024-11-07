use actix_web::{get, web::Data, HttpResponse, Responder};

use sea_orm::entity::*;
use sea_orm::prelude::*;

use crate::entities::{prelude::*, *};
use std::sync::Arc;

// #[get("/data/countries")]
// async fn all_countries(db: Data<DatabaseConnection>) -> impl Responder {
//     // query via SeaORM
//     // let result = Countries::find().all(&**db).await;
//     let result = Countries::find().all(db.as_ref()).await;

//     match result {
//         Ok(records) => HttpResponse::Ok().json(records),
//         Err(err) => {
//             HttpResponse::InternalServerError().body(format!("Database query failed: {:?}", err))
//         }
//     }

//     // HttpResponse::Ok().json("Country list")
// }

// #[get("/data/countries")]
// async fn all_countries(db: Data<Arc<DatabaseConnection>>) -> impl Responder {
//     // let result = Countries::find().all(&**db).await;
//     let result = Countries::find().all(db.as_ref().as_ref()).await;

//     match result {
//         Ok(records) => HttpResponse::Ok().json(records),
//         Err(err) => {
//             eprintln!("Error fetching countries: {:?}", err);
//             HttpResponse::InternalServerError().body("Database error")
//         }
//     }
// }

#[get("/data/countries")]
async fn all_countries(db: Data<DatabaseConnection>) -> impl Responder {
    // let result = Countries::find().all(&**db).await;
    let result = Countries::find().all(db.as_ref()).await;

    match result {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(err) => {
            eprintln!("Error fetching countries: {:?}", err);
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}
