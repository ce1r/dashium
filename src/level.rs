use axum::Router;
use axum::routing::post;

mod deleteGJLevelUser20;
mod downloadGJLevel22;
mod getGJLevels21;
mod suggestGJStars20;
mod uploadGJLevel21;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/uploadGJLevel21.php",
            post(uploadGJLevel21::uploadGJLevel21),
        )
        .route("/getGJLevels21.php", post(getGJLevels21::getGJLevels21))
        .route(
            "/downloadGJLevel22.php",
            post(downloadGJLevel22::downloadGJLevel22),
        )
        .route(
            "/deleteGJLevelUser20.php",
            post(deleteGJLevelUser20::deleteGJLevelUser20),
        )
        .route(
            "/suggestGJStars20.php",
            post(suggestGJStars20::suggestGJStars20),
        )
}
