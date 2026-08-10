use axum::Router;
use axum::routing::post;

mod getGJCommentHistory;
mod getGJComments21;
mod uploadGJComment21;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/getGJComments21.php",
            post(getGJComments21::getGJComments21),
        )
        .route(
            "/uploadGJComment21.php",
            post(uploadGJComment21::uploadGJComment21),
        )
        .route(
            "/getGJCommentHistory.php",
            post(getGJCommentHistory::getGJCommentHistory),
        )
}
