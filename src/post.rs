use axum::Router;
use axum::routing::post;

mod uploadGJAccComment20;

pub fn routes() -> Router {
    Router::new().route(
        "/uploadGJAccComment20.php",
        post(uploadGJAccComment20::uploadGJAccComment20),
    )
}
