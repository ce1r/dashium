use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::level::delete_level;
use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    levelID: i32,
}

pub async fn deleteGJLevelUser20(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let level_id = delete_level()
        .bind(&client, &form.levelID, &form.accountID)
        .one()
        .await?;

    let path = format!("data/levels/{level_id}.level");
    fs::remove_file(path).await?;

    Ok("1")
}
