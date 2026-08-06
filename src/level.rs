use axum::Router;
use axum::routing::post;

mod getGJLevels21;
mod uploadGJLevel21;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/uploadGJLevel21.php",
            post(uploadGJLevel21::uploadGJLevel21),
        )
        .route("/getGJLevels21.php", post(getGJLevels21::getGJLevels21))
}
