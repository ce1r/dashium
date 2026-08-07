use axum::Router;
use axum::routing::get;

mod level;
mod user;

pub fn routes() -> Router {
    Router::new()
        .route("/user/{username}", get(user::get))
        .route("/level/{level_id}", get(level::get))
}
