use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// 1. Ini adalah Entitas utama yang merepresentasikan tabel "users"
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")] // Nama tabel di MySQL
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32, // MySQL biasanya menggunakan i32 untuk ID
    pub username: String,
}

// 2. Definisi relasi antar tabel (saat ini kosong karena belum ada tabel lain)
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// 3. DTO (Data Transfer Object) khusus untuk menerima request POST (tanpa ID)
#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
}