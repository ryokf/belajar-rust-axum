use axum::Json;
use crate::models::user::User;

pub async fn get_user() -> Json<User> {
    let user = User{
        id: 1,
        name: "ryo".to_string(),
        job: "programmer".to_string(),
        age: 20
    };

    Json(user)
}