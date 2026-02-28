use axum::{ Router, routing::get };
use crate::{controllers::user_controller::*, state::AppState};

pub fn api_router(state: AppState) -> Router {
    let user_routes = Router::new().route("/", get(get_user).post(post_user)).route("/{id}", get(get_user_by_id));

    Router::new().nest("/user", user_routes).with_state(state)
}
