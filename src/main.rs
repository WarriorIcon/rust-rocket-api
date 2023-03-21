#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use rocket::serde::json::{Value, json};
use rocket::response::status;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

// test routes with curl 127.0.0.1:8000/rustaceans/1 -X DELETE -H 'Content-type: application/json'
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, _db: DbConn) -> Value {
    db.run(|c| {
        schema::rustaceans::table.limit(100).load.().expect("Failed to read db")
    }).await
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean
        ])
        .register("/", catchers![
            not_found,
            unauthorized,
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
