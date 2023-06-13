// Enable the necessary features
#![feature(proc_macro_hygiene, decl_macro)]

// Import the rocket crate
#[macro_use] extern crate rocket;

// Import necessary modules from rocket_contrib and serde
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

// Define a User struct that can be serialized and deserialized
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Define a GET route at "/users" that returns a list of users
#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    // Create some example users
    let user1 = User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() };
    let user2 = User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() };
    
    // Create a vector of users and return it as JSON
    let users = vec![user1, user2];
    Json(users)
}

// Define a POST route at "/users" that accepts JSON data
#[post("/users", format = "json", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    // Log the received user
    println!("User: {:?}", user);
    
    // Return the received user
    user
}

// Create a function to launch the Rocket instance
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        // Mount the defined routes at the root ("/") path
        .mount("/", routes![get_users, create_user])
}

// Define the main function that launches the rocket
fn main() {
    rocket().launch();
}
