use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::contentType, StatusCode}
};

use serde::{Serialize, Deserialize};
use derive_more::Display;

#[get("/task/{task_globa_id}")]
pub async fn get_task() -> Json<String> {

Json("hellow world.".to_string());
}



