// Import necessary libraries
use diesel::pg::PgConnection; // Database connection type for PostgreSQL.
use diesel::prelude::*; // Bring all of Diesel's functions into scope.
use dotenv::dotenv; // Loads environment variables from .env file.
use std::env; // Access to environment variables.

// Publicly expose modules so other parts of the program can use them.
pub mod orders;
pub mod products;
pub mod stock;

// Define a trait with the necessary methods to manipulate items in the database.
pub trait Item<T> {
    fn insert(); // Method for inserting an item into the database.

    fn delete(); // Method for deleting an item from the database.
}

// Function to establish a connection to the PostgreSQL database.
pub fn get_connection() -> PgConnection {
    // Load the environment variables from .env file.
    dotenv().ok();

    // Fetch the database URL from the environment variables.
    // If the DATABASE_URL environment variable is not set, this will panic.
    let db_url = env::var("DATABASE_URL").expect("DB path not set");

    // Establish a connection to the database at the given URL.
    // If a connection cannot be established, this will panic.
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}
