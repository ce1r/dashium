use axum::Router;
use axum::routing::post;

mod blockGJUser20;
mod deleteGJMessages20;
mod downloadGJMessage20;
mod getGJFriendRequests20;
mod getGJMessages20;
mod unblockGJUser20;
mod uploadFriendRequest20;
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
        .route(
            "/getGJMessages20.php",
            post(getGJMessages20::getGJMessages20),
        )
        .route(
            "/downloadGJMessage20.php",
            post(downloadGJMessage20::downloadGJMessage20),
        )
        .route(
            "/deleteGJMessages20.php",
            post(deleteGJMessages20::deleteGJMessages20),
        )
        .route(
            "/uploadFriendRequest20.php",
            post(uploadFriendRequest20::uploadFriendRequest20),
        )
        .route(
            "/getGJFriendRequests20.php",
            post(getGJFriendRequests20::getGJFriendRequests20),
        )
}
