mod controllers;
mod models;
mod routes;
mod state;

use dotenvy::dotenv;
use sea_orm::Database;
use std::env;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL harus diatur di .env");

    let db = Database::connect(&db_url).await.expect("Gagal terhubung ke database");

    let app_state = AppState{db};

    let app = routes::user_route::api_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server berjalan di http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
