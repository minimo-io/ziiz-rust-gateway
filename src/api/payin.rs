use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{ header::ContentType, StatusCode }
};

use serde::{Serialize, Deserialize};
// this implement the to_string method for enums but we can do this with strum:
// https://docs.rs/strum/latest/strum/
// use derive_more::{Display}; 

// test payin
#[get("/payin/pay")]
pub async fn pay() -> Json<String>{
    // format!("Hello my dear ziiz friend!");
    return Json("Hello, let's pay!".to_string());

}






// DELETE: First example
// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     format!("Hello {name}!")
// }