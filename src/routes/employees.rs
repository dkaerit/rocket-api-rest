use crate::note::Note;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
#[get("/employees")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Employee {
            id: 1,
            first_name : "Ola".to_string(),
            last_name: "John Ajiboye".to_string(),
           department: "Engineering".to_string(),
           salary: 4500,
           age: 23
        },
       Emoloyee {
             id: 2,
            first_name : "James".to_string(),
            last_name: "Bond".to_string(),
           department: "Accounting".to_string(),
           salary: 3500,
        