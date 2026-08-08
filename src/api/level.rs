use crate::Database;
use crate::Result;
use crate::error::AppError;
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use cornucopia::queries::level::get_level;
use cornucopia::queries::level::get_level_count;
use cornucopia::types::Visibility;

pub async fn get(Path(level_id): Path<i32>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let level = get_level().bind(&client, &level_id).one().await?;

    if level.visibility != Visibility::Public {
        return Err(AppError::Unhandled);
    }

    Ok(Json(level))
}

pub async fn count() -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let count = get_level_count().bind(&client).one().await?;

    Ok(Json(count))
}
