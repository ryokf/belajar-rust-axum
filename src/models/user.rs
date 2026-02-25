use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub job: String,
    pub age: i32
}