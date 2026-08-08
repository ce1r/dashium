use crate::Database;
use crate::Result;
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use cornucopia::queries::level::get_levels_of_user;
use cornucopia::queries::user::get_user_by_id;
use cornucopia::queries::user::get_user_count;

pub async fn get(Path(user_id): Path<i32>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let user = get_user_by_id().bind(&client, &user_id).one().await?;

    Ok(Json(user))
}

pub async fn get_levels(Path(user_id): Path<i32>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let user = get_levels_of_user().bind(&client, &user_id).all().await?;

    Ok(Json(user))
}

pub async fn count() -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let count = get_user_count().bind(&client).one().await?;

    Ok(Json(count))
}
