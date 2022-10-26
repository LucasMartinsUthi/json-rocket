mod api; 
mod models;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, get_user};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
}