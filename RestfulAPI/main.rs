#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let user1 = User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() };
    let user2 = User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() };
    let users = vec![user1, user2];
    Json(users)
}

#[post("/users", format = "json", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    println!("User: {:?}", user);
    user
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get_users, create_user])
}

fn main() {
    rocket().launch();
}
