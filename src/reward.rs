use axum::Router;
use axum::routing::post;

mod getGJRewards;

pub fn routes() -> Router {
    Router::new().route("/getGJRewards.php", post(getGJRewards::getGJRewards))
}
