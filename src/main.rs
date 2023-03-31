#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod schema;

use diesel::prelude::*;
use auth::BasicAuth;
use models::{Rustacean, NewRustacean};
use schema::rustaceans;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

// test routes with curl 127.0.0.1:8000/rustaceans/1 -X DELETE -H 'Content-type: application/json'
// test route with curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
// .limit limits the query to x records. .load translates the records into the rustacean model
// .expect() a panic with an error message if reading the DB fails.
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = rustaceans::table.limit(100).load::<Rustacean>(c).expect("Failed to read db");
        json!(result)
    }).await
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
        .values(new_rustacean.into_inner())
        .execute(c)
        .expect("Failed Inserting rustacean entry");
        // return resukt
        json!(result)
    }).await
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
