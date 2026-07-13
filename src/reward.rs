use axum::Router;
use axum::routing::post;

mod getGJChallenges;
mod getGJRewards;

pub fn routes() -> Router {
    Router::new()
        .route("/getGJRewards.php", post(getGJRewards::getGJRewards))
        .route(
            "/getGJChallenges.php",
            post(getGJChallenges::getGJChallenges),
        )
}
