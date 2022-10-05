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
    rocket::build().mount("/", routes![routes::list_customers])
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
}
