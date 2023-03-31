// All the database columns will be translated as Rust fields in this struct via Diesel
use super::schema::rustaceans;
#[derive(serde::Serialize,Queryable,serde::Deserialize)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}
#[derive(serde::Deserialize, Insertable)]
#[table_name="rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}