use actix_web::{get, HttpResponse, Responder, web::Data};
use sea_orm::entity::*;
use sea_orm::prelude::*;

#[get("/countries")]
async fn countries(db: Data<DbConn>) -> impl Responder {

    // query via SeaORM
    //let result = EntityName::find().all(&*db).await;

    // match result {
    //     Ok(records) => HttpResponse::Ok().json(records),
    //     Err(err) => HttpResponse::InternalServerError().body(format!("Database query failed: {:?}", err)),
    // } 

    HttpResponse::Ok().json("Country list")
    
}