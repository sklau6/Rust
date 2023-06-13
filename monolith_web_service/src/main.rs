// Import external libraries.
#[macro_use]
extern crate diesel;
extern crate dotenv;

// Import internal modules.
mod api;
mod data;
mod models;
mod schema;
mod services;
mod web_service;

use crate::web_service::WebService; // Import the web service.

// Uncomment the line below if you need to use the actix_web library.
// use actix_web::{web, App, HttpServer};

/// The main function that runs the web server and sets up all the API endpoints.
///
/// Endpoint summary:
///     - Create a new order: /order/create
///     - Cancel an Order: /order/cancel
///     - Update an Order: /order/update
///     - Fulfill a specific Order: /order/fulfill
///     - Create a product Order: /product/create
///     - Delete a Product: /product/delete
///     - Update a Product: /product/update
///     - Create Stock: /stock/create
///     - Delete Stock: /stock/delete
///     - Update Stock: /stock/update
///     - Increment the amount of stock: /stock/increment
///
/// Each endpoint corresponds to a specific action that can be performed on the
/// server's data. For example, /order/create can be used to create a new order, while
/// /order/cancel can be used to cancel an existing order.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut web_service = WebService::new(); // Create a new instance of the web service.
    web_service.start_webserver().await // Start the web server.
}
