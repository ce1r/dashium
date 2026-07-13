use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use cornucopia::queries::user::update_settings;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    mS: i16,
    frS: i16,
    cS: i16,
    yt: String,
    twitter: String,
    twitch: String,
    discord: String,
    instagram: String,
    tiktok: String,
}

pub async fn updateGJAccSettings20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    update_settings()
        .bind(
            &client,
            &form.mS,
            &form.frS,
            &form.cS,
            &form.yt,
            &form.twitter,
            &form.twitch,
            &form.discord,
            &form.instagram,
            &form.tiktok,
            &form.accountID,
        )
        .await?;

    Ok("1".to_string())
}
