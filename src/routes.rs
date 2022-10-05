use crate::models::*;
use crate::db::*;
use rocket::response::status;
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

#[get("/customer/<guid>")]
pub async fn get_customer(guid: String) -> Result<Json<CustomerResponse>, status::NotFound<String>> {
    let db = init_db();
    let customers = db.lock().await;
    for customer in customers.to_vec() {
        if customer.guid == guid {
            return Ok (Json(CustomerResponse { customer }))
        }
    }
    Err(status::NotFound(format!("Error getting customer {}.", guid)))
}