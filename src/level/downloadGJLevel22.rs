use crate::Database;
use crate::Result;
use crate::gd_format;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use chrono_humanize::HumanTime;
use cornucopia::queries::level::Level;
use cornucopia::queries::level::get_level;
use cornucopia::types::DemonDifficulty;
use cornucopia::types::LevelLength;
use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Data {
    levelID: i32,
}

pub async fn downloadGJLevel22(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let level = get_level().bind(&client, &form.levelID).one().await?;

    let path = format!("data/levels/{}.level", level.id);
    let level_string = URL_SAFE.encode(fs::read(path).await?);

    let hash1 = generate_hash1(&level_string);
    let hash2 = generate_hash2(&level, 0);

    let length = match level.length {
        LevelLength::Tiny => 0,
        LevelLength::Short => 1,
        LevelLength::Medium => 2,
        LevelLength::Long => 3,
        LevelLength::XL => 4,
    };

    let demon_difficulty = match level.demon_difficulty {
        Some(DemonDifficulty::Easy) => 3,
        Some(DemonDifficulty::Medium) => 4,
        Some(DemonDifficulty::Hard) | None => 0,
        Some(DemonDifficulty::Insane) => 5,
        Some(DemonDifficulty::Extreme) => 6,
    };

    let created_at = HumanTime::from(level.created_at)
        .to_string()
        .replace(" ago", "");

    let response = gd_format!(
        ":",
        1 => level.id,
        2 => level.name,
        4 => level_string,
        6 => level.user_id,
        8 => u8::from(level.is_featured),
        10 => level.downloads,
        12 => level.official_song_id,
        14 => level.likes,
        15 => length,
        16 => level.dislikes,
        17 => u8::from(level.is_demon),
        18 => level.stars,
        25 => u8::from(level.is_auto),
        27 => 0,
        28 => created_at,
        29 => created_at,
        31 => u8::from(level.is_two_player),
        35 => level.song_id,
        37 => level.coins,
        38 => u8::from(level.has_verified_coins),
        39 => level.requested_stars,
        40 => u8::from(level.is_ldm),
        43 => demon_difficulty,
        44 => 0,
        45 => level.objects,
        62 => level.created_at.timestamp(),
    );

    Ok(format!("{response}#{hash1}#{hash2}"))
}

pub fn generate_hash1(level_string: &str) -> String {
    if level_string.len() < 41 {
        return util::salt_and_sha1(level_string, "xI25fpAapCQg");
    }

    let mut hash_chars = "????????????????????????????????????????xI25fpAapCQg"
        .chars()
        .collect::<Vec<_>>();

    let m = level_string.len() / 40;

    for i in (0..40).rev() {
        if let Some(c) = level_string.chars().nth(i * m) {
            hash_chars[i] = c;
        }
    }

    let hash = hash_chars.into_iter().collect::<String>();

    util::salt_and_sha1(&hash, "")
}

pub fn generate_hash2(level: &Level, daily_id: i32) -> String {
    let hash_input = format!(
        "{},{},{},{},{},{},{},{}",
        level.user_id,
        level.stars,
        u8::from(level.is_demon),
        level.id,
        u8::from(level.has_verified_coins),
        0,
        0,
        daily_id
    );

    util::salt_and_sha1(&hash_input, "xI25fpAapCQg")
}
