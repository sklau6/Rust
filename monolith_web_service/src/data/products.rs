// Importing necessary modules and traits
use crate::data::get_connection;
use crate::models::{NewProduct, Product};
use crate::schema;
use crate::schema::products::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

// Function to create a new product in the database
pub fn create_product<'a>(product: &'a Product) -> Product {
    use schema::products;

    // Establishing a connection to the database
    let conn = get_connection();

    // Creating a new product instance
    let new_product = NewProduct {
        product_name: product.product_name.clone(),
        product_type: product.product_type.clone(),
        amount: product.amount,
    };

    // Inserting the new product into the database and returning the result
    let ret = diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(&conn)
        .expect("Error saving new product");

    println!("Ret: {:?}", ret);

    ret
}

// Function to update an existing product in the database
pub fn update_product<'a>(con: &PgConnection, product: &'a Product) {
    // Updating the product
    let product = diesel::update(products)
        .set(product)
        .get_result::<Product>(con)
        .expect(&format!("Unable to find product {}", product.id));

    println!("Updated product {}", product.id);
}

// Function to delete a product from the database
pub fn delete_product<'a>(con: &PgConnection, product: &'a Product) {
    // Deleting the product and returning the number of deleted products
    let num_deleted = diesel::delete(products.find(product.id))
        .execute(con)
        .expect("Error deleting products");

    println!("Deleted {} products", num_deleted);
}

// Function to fetch products from the database
pub fn show_products() -> Vec<Product> {
    // Establishing a connection to the database
    let connection = get_connection();

    // Loading products from the database
    let results = products
        .limit(10)
        .load::<Product>(&connection)
        .expect("Error loading products");

    results
}
