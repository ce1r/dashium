use axum::Router;
use axum::routing::post;

mod registerGJAccount;

pub fn routes() -> Router {
    Router::new().route(
        "/accounts/registerGJAccount.php",
        post(registerGJAccount::registerGJAccount),
    )
}
