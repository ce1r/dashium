use axum::Router;
use axum::routing::post;

mod uploadGJLevel21;

pub fn routes() -> Router {
    Router::new().route(
        "/uploadGJLevel21.php",
        post(uploadGJLevel21::uploadGJLevel21),
    )
}
