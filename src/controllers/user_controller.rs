use axum::{Json, extract::{Path, State}};
use sea_orm::{ActiveModelTrait, EntityTrait, ActiveValue::Set};
use crate::{
    models::user::{ActiveModel as UserActiveModel, CreateUserDto, Entity as UserEntity, Model as UserModel},
    state::AppState
};

pub async fn get_user(State(state): State<AppState>) -> Json<Vec<UserModel>> {
    let user = UserEntity::find().all(&state.db).await.unwrap();

    Json(user)
}

pub async fn get_user_by_id(Path(id): Path<i32>) -> Json<i32> {
    Json(id)
}

pub async fn post_user(State(state): State<AppState>, Json(request): Json<CreateUserDto>) -> Json<UserModel>{
    let new_user = UserActiveModel{
        username: Set(request.username),
        ..Default::default()
    };

    let inserted_user = new_user.insert(&state.db).await.unwrap();

    Json(inserted_user)
}