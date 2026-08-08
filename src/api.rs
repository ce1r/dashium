use axum::Router;
use axum::routing::get;

mod level;
mod user;

pub fn routes() -> Router {
    Router::new()
        .route("/users/{user_id}", get(user::get))
        .route("/users/count", get(user::count))
        .route("/users/{user_id}/levels", get(user::get_levels))
        .route("/levels/{level_id}", get(level::get))
        .route("/levels/count", get(level::count))
}
