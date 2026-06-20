use axum::Router;
use axum::routing::post;

mod backupGJAccountNew;
mod getAccountURL;
mod getGJUserInfo20;
mod loginGJAccount;
mod registerGJAccount;
mod requestUserAccess;
mod syncGJAccountNew;
mod updateGJAccSettings20;
mod updateGJUserScore22;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/accounts/registerGJAccount.php",
            post(registerGJAccount::registerGJAccount),
        )
        .route(
            "/accounts/loginGJAccount.php",
            post(loginGJAccount::loginGJAccount),
        )
        .route(
            "/database/accounts/backupGJAccountNew.php",
            post(backupGJAccountNew::backupGJAccountNew),
        )
        .route("/getAccountURL.php", post(getAccountURL::getAccountURL))
        .route(
            "/database/accounts/syncGJAccountNew.php",
            post(syncGJAccountNew::syncGJAccountNew),
        )
        .route(
            "/getGJUserInfo20.php",
            post(getGJUserInfo20::getGJUserInfo20),
        )
        .route(
            "/updateGJUserScore22.php",
            post(updateGJUserScore22::updateGJUserScore22),
        )
        .route(
            "/requestUserAccess.php",
            post(requestUserAccess::requestUserAccess),
        )
        .route(
            "/updateGJAccSettings20.php",
            post(updateGJAccSettings20::updateGJAccSettings20),
        )
}
