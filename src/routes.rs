use crate::models::*;
use crate::db::*;
use rocket::response::status;
use rocket::http::Status;
use rocket::serde::json::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerListResponse {
    pub customers: Vec<Customer>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerResponse {
    pub customer: Customer,
}

#[get("/customers")]
pub async fn list_customers() -> Json<CustomerListResponse> {
    let db = init_db();
    let customers = db.lock().await;
    Json(CustomerListResponse { customers: customers.to_vec() })
}

#[post("/customers", format = "json", data = "<customer_json>")]
pub async fn add_customer(customer_json: Json<Customer>) -> Status {
    let db = init_db();
    let customers = db.lock().await;
    let new_customer = customer_json.into_inner();
    for customer in customers.to_vec() {
        if customer.guid == new_customer.guid {
            return Status::BadRequest
        }
    }
    // NOTE: we should have updated the database here.
    Status::Ok
}

#[get("/customer/<guid>")]
pub async fn list_customer(guid: String) -> Result<Json<CustomerResponse>, status::NotFound<String>> {
    let db = init_db();
    let customers = db.lock().await;
    for customer in customers.to_vec() {
        if customer.guid == guid {
            return Ok (Json(CustomerResponse { customer }))
        }
    }
    Err(status::NotFound(format!("Error getting customer {}.", guid)))
}

#[delete("/customer/<guid>")]
pub async fn delete_customer(guid: String) -> Result<Json<CustomerResponse>, status::NotFound<String>> {
    let db = init_db();
    let customers = db.lock().await;
    for customer in customers.to_vec() {
        if customer.guid == guid {
            return Ok (Json(CustomerResponse { customer }))
        }
    }
    Err(status::NotFound(format!("Error getting customer {}.", guid)))
}