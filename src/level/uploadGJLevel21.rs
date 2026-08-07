use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::level::create_level;
use cornucopia::types::LevelLength;
use cornucopia::types::Visibility;
use serde::Deserialize;
use serde_with::BoolFromInt;
use serde_with::serde_as;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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

    levelLength: u8,
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

    unlisted: u8,
}

pub async fn uploadGJLevel21(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let description_bytes = URL_SAFE.decode(&form.levelDesc)?;
    let description = String::from_utf8(description_bytes)?;

    let is_platformer = form.levelLength == 5;

    let level_length = match form.levelLength {
        1 => LevelLength::Short,
        2 => LevelLength::Medium,
        3 => LevelLength::Long,
        4 => LevelLength::XL,
        _ => LevelLength::Tiny,
    };

    let visibility = match form.unlisted {
        1 => Visibility::FriendsOnly,
        2 => Visibility::Private,
        _ => Visibility::Public,
    };

    let level_id = create_level()
        .bind(
            &client,
            &form.levelName,
            &description,
            &form.levelVersion,
            &form.original,
            &level_length,
            &form.objects,
            &form.requestedStars,
            &form.coins,
            &form.auto,
            &form.ldm,
            &form.twoPlayer,
            &is_platformer,
            &form.audioTrack,
            &form.songID,
            &visibility,
            &form.accountID,
        )
        .one()
        .await?;

    let level_string = URL_SAFE.decode(form.levelString)?;

    let path = format!("data/levels/{level_id}.level");
    let mut file = File::create(path).await?;
    file.write_all(&level_string).await?;

    Ok(level_id.to_string())
}
