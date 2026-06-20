use crate::Database;
use crate::Result;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::level::create_level;
use serde::Deserialize;
use serde_with::BoolFromInt;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    levelName: String,
    levelDesc: String,
    levelString: String,

    levelVersion: i32,
    original: i32,

    levelLength: i16,
    objects: i32,
    requestedStars: i16,
    coins: i16,

    #[serde_as(as = "BoolFromInt")]
    auto: bool,

    #[serde_as(as = "BoolFromInt")]
    ldm: bool,

    #[serde_as(as = "BoolFromInt")]
    twoPlayer: bool,

    audioTrack: i32,
    songID: i32,

    unlisted: i16,
}

pub async fn uploadGJLevel21(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    let description_bytes = URL_SAFE.decode(&form.levelDesc)?;
    let description = String::from_utf8(description_bytes)?;

    let level_id = create_level()
        .bind(
            &client,
            &form.levelName,
            &description,
            &form.levelString,
            &form.levelVersion,
            &form.original,
            &form.levelLength,
            &form.objects,
            &form.requestedStars,
            &form.coins,
            &form.auto,
            &form.ldm,
            &form.twoPlayer,
            &form.audioTrack,
            &form.songID,
            &form.unlisted,
            &form.accountID,
            &form.gjp2,
        )
        .one()
        .await?;

    Ok(level_id.to_string())
}
