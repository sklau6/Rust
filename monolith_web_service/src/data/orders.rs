// Import necessary modules and structures
use crate::data::get_connection;
use crate::models::{NewOrder, Order};
use crate::schema;
use crate::schema::orders::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

// Function to create a new order in the database
pub fn create_order<'a>(conn: &PgConnection, order: &'a Order) -> Order {
    use schema::orders;

    // Create a new order object from the given data
    let new_order = NewOrder {
        product_name: order.product_name.clone(),
        product_id: order.product_id,
        customer_id: order.customer_id,
        amount: order.amount,
        address: order.address.clone(),
    };

    // Insert the new order into the database
    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(conn)
        .expect("Error saving new order")
}

// Function to fulfill an order in the database
pub fn fulfill_order<'a>(con: &PgConnection, order_id: i32) -> Option<Order> {
    // Update the order's fulfilled field to true
    let order = diesel::update(orders.find(order_id))
        .set(fulfilled.eq(true))
        .get_result::<Order>(con)
        .expect(&format!("Unable to find order {}", order_id));

    // Log the action
    println!("Fulfilled order {}", order.id);

    // Return the updated order
    Some(order)
}

// Function to update an existing order in the database
pub fn update_order<'a>(con: &PgConnection, order: &'a Order) -> Order {
    // Update the order in the database
    let order = diesel::update(orders)
        .set(order)
        .get_result::<Order>(con)
        .expect(&format!("Unable to find order {}", order.id));

    // Log the action
    println!("Updated order {}", order.id);

    // Return the updated order
    order
}

// Function to delete an order in the database
pub fn delete_order<'a>(con: &PgConnection, order: &'a Order) -> usize {
    // Delete the order from the database
    let num_deleted = diesel::delete(orders.find(order.id))
        .execute(con)
        .expect("Error deleting orders");

    // Log the action
    println!("Deleted {} orders", num_deleted);

    // Return the number of deleted orders
    num_deleted
}

// Function to display orders of a specific customer
pub fn show_orders(customer_id_needed: i32) -> Vec<Order> {
    // Get a connection to the database
    let connection = get_connection();

    // Query the database for orders of the specific customer, limited to 5 orders
    let results = orders
        .filter(customer_id.eq(customer_id_needed))
        .limit(5)
        .load::<Order>(&connection)
        .expect("Error loading orders");

    // Return the found orders
    results
}
