use crate::models::*;
use crate::db::*;
//use rocket::response::content::Json;
use rocket::serde::json::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerListResponse {
    pub customers: Vec<Customer>,
}

#[get("/customers")]
pub async fn list_customers() -> Json<CustomerListResponse> {
    let db = init_db();
    let customers = db.lock().await;
    Json(CustomerListResponse { customers: customers.to_vec() })
}