use axum::Router;
use axum::routing::get;

mod level;
mod user;

pub fn routes() -> Router {
    Router::new()
        .route("/users/{username}", get(user::get))
        .route("/users/{username}/levels", get(user::get_levels))
        .route("/levels/{level_id}", get(level::get))
}
