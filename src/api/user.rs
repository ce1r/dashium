use crate::Database;
use crate::Result;
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use cornucopia::queries::user::get_user_by_username;

pub async fn get(Path(username): Path<String>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let user = get_user_by_username()
        .bind(&client, &username)
        .one()
        .await?;

    Ok(Json(user))
}
