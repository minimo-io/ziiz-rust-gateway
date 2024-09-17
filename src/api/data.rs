use actix_web::{get, HttpResponse, Responder, web::Data};

use sea_orm::entity::*;
use sea_orm::prelude::*;

use crate::entity::{prelude::*, *};

#[get("/data/countries")]
async fn all_countries(db: Data<DatabaseConnection>) -> impl Responder {

    // query via SeaORM
    // let result = Countries::find().all(&*db).await?;
    let result = Countries::find().all(&**db).await;

    match result {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(err) => HttpResponse::InternalServerError().body(format!("Database query failed: {:?}", err)),
    }

    // HttpResponse::Ok().json("Country list")
    
}