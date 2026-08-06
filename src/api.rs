use axum::Router;
use axum::routing::get;

mod user;

pub fn routes() -> Router {
    Router::new().route("/user/{username}", get(user::get_user))
}
