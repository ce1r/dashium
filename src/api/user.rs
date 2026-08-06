use crate::Database;
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use cornucopia::queries::user::get_user_by_username;

pub async fn get_user(Path(username): Path<String>) -> impl IntoResponse {
    let client = Database::acquire().await.unwrap();

    let user = get_user_by_username()
        .bind(&client, &username)
        .one()
        .await
        .unwrap();

    Json(user)
}
