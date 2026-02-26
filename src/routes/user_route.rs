use axum::{Router, routing::{get, post}};
use crate::controllers::user_controller::*;

pub fn api_router() -> Router{
    let user_routes = Router::new().route("/", get(get_user)).route("/", post(post_user));

    Router::new().nest("/user", user_routes)
}