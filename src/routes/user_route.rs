use axum::{routing::get, Router};
use crate::controllers::user_controller::*;

pub fn api_router() -> Router{
    Router::new().route("/", get(get_user))
}