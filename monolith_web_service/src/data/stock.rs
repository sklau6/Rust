// Importing necessary modules and functions
use crate::data::get_connection;
use crate::models::{NewStock, Stock};
use crate::schema;
use crate::schema::stocks::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::stocks;

// Function to create a new stock
pub fn create_stock<'a>(conn: &PgConnection, stock: &'a Stock) -> Stock {
    // Creating a new stock object from the input
    let new_stock = NewStock {
        product_name: stock.product_name.clone(),
        product_id: stock.product_id,
        amount: stock.amount,
    };

    // Inserting the new stock into the stocks table and getting the result
    let ret = diesel::insert_into(stocks::table)
        .values(&new_stock)
        .get_result(conn)
        .expect("Error saving new stock");

    // Print the result
    println!("Created stock: {:?}", ret);

    // Return the created stock
    ret
}

/// Function to increment the amount of a stock
///
/// # Arguments
///
/// * `con`          - Connection to the database
/// * `stock_id`     - ID of the stock to increment
/// * `amount_change`- The number to increment the stock by
pub fn increment_stock<'a>(con: &PgConnection, stock_id: i32, amount_change: i32) -> Stock {
    // Updating the stock with the given id and getting the result
    let stock = diesel::update(stocks.find(stock_id))
        .set(amount.eq(amount + amount_change))
        .get_result::<Stock>(con)
        .expect(&format!("Unable to find stock {}", stock_id));

    // Print the updated stock
    println!("Incremented stock: {:?}", stock);

    // Return the updated stock
    stock
}

// Function to update a stock
pub fn update_stock<'a>(con: &PgConnection, stock: &'a Stock) -> Stock {
    // Updating the stock and getting the result
    let stock = diesel::update(stocks)
        .set(stock)
        .get_result::<Stock>(con)
        .expect(&format!("Unable to find stock {}", stock.id));

    // Print the ID of the updated stock
    println!("Updated stock {}", stock.id);

    // Return the updated stock
    stock
}

// Function to delete a stock
pub fn delete_stock<'a>(con: &PgConnection, stock: &'a Stock) -> usize {
    // Deleting the stock with the given id and getting the number of deleted stocks
    let num_deleted = diesel::delete(stocks.find(stock.id))
        .execute(con)
        .expect("Error deleting stocks");

    // Print the number of deleted stocks
    println!("Deleted {} stocks", num_deleted);

    // Return the number of deleted stocks
    num_deleted
}

// Function to show the stocks
pub fn show_stock() -> Vec<Stock> {
    // Getting the connection
    let connection = get_connection();

    // Loading the stocks and getting the result
    let results = stocks
        .limit(10)
        .load::<Stock>(&connection)
        .expect("Error loading stocks");

    // Return the stocks
    results
}

// Function to get a stock with the given id
pub fn get_stock(con: &PgConnection, stock_id: i32) -> Vec<Stock> {
    // Filtering the stocks with the given id, loading them, and getting the result
    let results = stocks
        .filter(id.eq(stock_id))
        .limit(5)
        .load::<Stock>(con)
        .expect("Error loading stocks");

    // Return the stocks
    results
}
