use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::Customer;

pub type Db = Arc<Mutex<Vec<Customer>>>;