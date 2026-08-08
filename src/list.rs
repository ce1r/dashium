use axum::Router;
use axum::routing::post;

mod uploadGJLevelList;

pub fn routes() -> Router {
    Router::new().route(
        "/uploadGJLevelList.php",
        post(uploadGJLevelList::uploadGJLevelList),
    )
}
