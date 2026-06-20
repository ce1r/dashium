use crate::Database;
use crate::Result;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::post::create_post;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    comment: String,
    gjp2: String,
}

pub async fn uploadGJAccComment20(Form(form): Form<Data>) -> Result<String> {
    let decoded = URL_SAFE.decode(&form.comment)?;
    let body = String::from_utf8(decoded)?;

    let client = Database::acquire().await?;

    let comment_id = create_post()
        .bind(&client, &form.accountID, &body, &form.gjp2)
        .one()
        .await?;

    Ok(comment_id.to_string())
}
