use axum::Router;
use axum::routing::post;

mod blockGJUser20;

pub fn routes() -> Router {
    Router::new().route("/blockGJUser20.php", post(blockGJUser20::blockGJUser20))
}
