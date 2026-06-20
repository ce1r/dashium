use axum::Router;
use axum::routing::post;

mod deleteGJAccComment20;
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
        .route(
            "/deleteGJAccComment20.php",
            post(deleteGJAccComment20::deleteGJAccComment20),
        )
}
