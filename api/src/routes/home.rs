use actix_web::{post, get, web::Json};
use serde::{Serialize, Deserialize};



#[get("/")]
pub async fn home() -> Json<String>{
    // format!("Hello my dear ziiz friend!");
    return Json(
        String::from("[{'result': 1}]")
    );

}
