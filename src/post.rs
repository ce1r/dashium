use axum::Router;
use axum::routing::post;

mod getGJAccountComments20;
mod uploadGJAccComment20;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/uploadGJAccComment20.php",
            post(uploadGJAccComment20::uploadGJAccComment20),
        )
        .route(
            "/getGJAccountComments20.php",
            post(getGJAccountComments20::getGJAccountComments20),
        )
}
