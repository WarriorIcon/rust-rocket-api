#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod schema;
mod repositories;

use auth::BasicAuth;
use models::{Rustacean, NewRustacean};
use rocket::serde::json::{Value, json, Json,};
use rocket::response::status;
use repositories::RustaceanRepository;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);


// .limit limits the query to x records. .load translates the records into the rustacean model
// .expect() a panic with an error message if reading the DB fails.
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = RustaceanRepository::find_multiple(c, 100)
        .expect("Failed to read db");
        json!(result)
    }).await
}

// test get route with curl 127.0.0.1:8000/rustaceans/1 -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let rustacean = RustaceanRepository::find(c, id)
        .expect("Failed retreiving rustacean row.");
        json!(rustacean)
    }).await
}

// Test API route with curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -H 'Content-type: application/json' -d '{"name": "Kate Locke", "email": "kate@locke.com"}'
#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = RustaceanRepository::create(c, new_rustacean.into_inner())
        .expect("Failed Inserting rustacean entry");
        // return result
        json!(result)
    }).await
}
// Test API with the following cmd. Be sure to change the id in the rustaceans/1 in the cmd to the id you want
// curl 127.0.0.1:8000/rustaceans/1 -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -X PUT -H 'Content-type: application/json' -d '{"name": "John Changed Doe", "email": "jon@dooooe.com"}'
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, db: DbConn, rustacean: Json<Rustacean>) -> Value {
    db.run(move |c| {
        let result = RustaceanRepository::save(c, id, rustacean.into_inner())
            // we only want to update these specific fields, NOT ID and Created_At via the Rustacean Struct
            // rustaceans:: here is the schema which are strings
            // .to_owned to switch these to owned structs - create a new string out them and pass the new string to eq
            .expect("Failed updating rustacean entry");
        json!(result)
    }).await
}
// test route with curl 127.0.0.1:8000/rustaceans/1 -X DELETE -H 'Content-type: application/json'
#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn,) -> status::NoContent {
    db.run(move |c| {
        // query for rustaceans record for the row with the specified id and call diesel delete on it
            RustaceanRepository::delete(c, id)
            .expect("DB error on deleting");
        status::NoContent
    }).await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized!")
}

#[catch(422)]
fn unprocessable() -> Value {
    json!({"error" : "422 - Invalid entity. Probably some missing fields?"})
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
            unprocessable,
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
