// All the database columns will be translated as Rust fields in this struct via Diesel
#[derive(serde::Serialize,Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}