use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::user::update_level_completion;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    levelID: i32,
    percent: u8,
}

pub async fn getGJLevelScores211(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    if form.percent > 100 {
        return Ok("-1");
    }

    let client = Database::acquire().await?;
    util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    update_level_completion()
        .bind(&client, &form.levelID, &form.accountID)
        .await?;

    Ok("1")
}
