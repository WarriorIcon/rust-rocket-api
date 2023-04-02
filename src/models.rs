// All the database columns will be translated as Rust fields in this struct via Diesel
use super::schema::rustaceans;
#[derive(serde::Serialize,Queryable, AsChangeset, serde::Deserialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)] // Prevent requirement to pass in a new ID for PUTS to the DB
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)] // Prevent requirement to pass in a new created_at for PUTS to the DB
    pub created_at: String, 
}
#[derive(serde::Deserialize, Insertable)]
#[table_name="rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}