#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

mod models;
mod db;
mod routes;

 use rocket::*;

fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![routes::list_customers, 
                                       routes::list_customer])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket()
        .ignite().await?
        .launch().await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn list_customers_returns_correct_results() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/customers")).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_customer_returns_correct_results_on_known_id() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/customer/4622d0f0-aad4-4c10-a6df-415232141866")).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_customer_returns_correct_results_on_unknown_id() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/customer/123")).dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
