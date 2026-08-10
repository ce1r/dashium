use axum::Router;
use axum::routing::post;

mod deleteGJLevelList;
mod getGJLevelLists;
mod uploadGJLevelList;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/uploadGJLevelList.php",
            post(uploadGJLevelList::uploadGJLevelList),
        )
        .route(
            "/getGJLevelLists.php",
            post(getGJLevelLists::getGJLevelLists),
        )
        .route(
            "/deleteGJLevelList.php",
            post(deleteGJLevelList::deleteGJLevelList),
        )
}
