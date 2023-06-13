// Imports schema definitions
use super::schema::orders;
use super::schema::products;
use super::schema::stocks;
use serde_derive::{Deserialize, Serialize};

// Represents an Order in the system
#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub customer_id: i32,
    pub amount: i32,
    pub address: String,
    pub fulfilled: bool,
}

// Order Implementation
impl Order {
    // Constructor for Order
    pub fn new(
        id: i32,
        product_name: String,
        product_id: i32,
        customer_id: i32,
        amount: i32,
        address: String,
    ) -> Order {
        Order {
            id,
            product_name,
            product_id,
            customer_id,
            amount,
            address,
            fulfilled: false,
        }
    }
}

// Represents a new Order to be inserted into the DB
#[derive(Insertable, Queryable)]
#[table_name = "orders"]
pub struct NewOrder {
    pub product_name: String,
    pub product_id: i32,
    pub customer_id: i32,
    pub amount: i32,
    pub address: String,
}

// Represents a Product in the system
#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub product_name: String,
    pub product_type: String,
    pub amount: i32,
}

// Represents a new Product to be inserted into the DB
#[derive(Insertable, Queryable)]
#[table_name = "products"]
pub struct NewProduct {
    pub product_name: String,
    pub product_type: String,
    pub amount: i32,
}

// Represents a Stock in the system
#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Stock {
    pub id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

// Represents a new Stock to be inserted into the DB
#[derive(Insertable, Queryable)]
#[table_name = "stocks"]
pub struct NewStock {
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

// Represents the return information structure
#[derive(Serialize, Deserialize)]
pub struct ReturnInfo {
    pub amount: usize,
}
