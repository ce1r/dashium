use axum::Router;
use axum::routing::post;

mod blockGJUser20;
mod unblockGJUser20;
mod uploadGJMessage20;

pub fn routes() -> Router {
    Router::new()
        .route("/blockGJUser20.php", post(blockGJUser20::blockGJUser20))
        .route(
            "/unblockGJUser20.php",
            post(unblockGJUser20::unblockGJUser20),
        )
        .route(
            "/uploadGJMessage20.php",
            post(uploadGJMessage20::uploadGJMessage20),
        )
}
