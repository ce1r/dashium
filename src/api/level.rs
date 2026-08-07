use crate::Database;
use crate::Result;
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use cornucopia::queries::level::get_level;

pub async fn get(Path(level_id): Path<i32>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let level = get_level().bind(&client, &level_id).one().await?;

    Ok(Json(level))
}
