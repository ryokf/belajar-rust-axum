use axum::{Json, extract::{Path, State}};
use chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, ActiveValue::Set};
use serde::Deserialize;
use crate::{
    models::users::{
        ActiveModel as UserActiveModel, Entity as UserEntity, Model as UserModel,
    },
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
}

pub async fn get_user(State(state): State<AppState>) -> Json<Vec<UserModel>> {
    let user = UserEntity::find().all(&state.db).await.unwrap();

    Json(user)
}

pub async fn get_user_by_id(Path(id): Path<i32>) -> Json<i32> {
    Json(id)
}

pub async fn post_user(State(state): State<AppState>, Json(request): Json<CreateUserDto>) -> Json<UserModel>{
    let today = Local::now().date_naive();

    let new_user = UserActiveModel{
        username: Set(request.username),
        created_at: Set(today),
        ..Default::default()
    };

    let inserted_user = new_user.insert(&state.db).await.unwrap();

    Json(inserted_user)
}