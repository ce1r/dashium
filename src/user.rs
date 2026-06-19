use axum::Router;
use axum::routing::post;

mod loginGJAccount;
mod registerGJAccount;

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
}
