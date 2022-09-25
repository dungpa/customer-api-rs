# customer-api-rs
A simple customer API written in Rust using Tokio and Rocket.
The work is inspired by https://github.com/andrewleverette/rust_warp_api.

```
/customers
    - GET -> list all customers in data store
    - POST -> create new customer and insert into data store
/customers/{guid}
    - GET -> list info for a customer
    - POST -> update information for a customer
    - DELETE -> remove customer from data store
```