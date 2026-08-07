use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::comment::create_comment;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    comment: String,
    levelID: i32,

    #[serde(default)]
    percent: i16,
}

pub async fn uploadGJComment21(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let comment = String::from_utf8(URL_SAFE.decode(&form.comment)?)?;
    if form.percent < 0 || form.percent > 100 {
        return Ok("-1".to_string());
    }

    let client = Database::acquire().await?;
    util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let comment_id = create_comment()
        .bind(
            &client,
            &form.accountID,
            &form.levelID,
            &comment,
            &form.percent,
        )
        .one()
        .await?;

    Ok(comment_id.to_string())
}
