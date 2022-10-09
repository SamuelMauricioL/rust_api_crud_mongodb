//add the modules
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

//add imports below
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
}
