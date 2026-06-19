use axum::Router;
use axum::routing::post;

mod backupGJAccountNew;
mod getAccountURL;
mod loginGJAccount;
mod registerGJAccount;
mod syncGJAccountNew;

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
}
