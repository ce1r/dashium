use crate::Database;
use crate::Result;
use crate::gd_format;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::level::Level;
use cornucopia::queries::level::search_levels;
use cornucopia::types::DemonDifficulty;
use cornucopia::types::LevelLength;
use serde::Deserialize;
use std::fmt::Write;

#[derive(Deserialize)]
pub struct Data {
    str: String,
    page: i64,
}

pub async fn getGJLevels21(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    let offset = form.page * 10;

    let levels = search_levels()
        .bind(&client, &form.str, &offset)
        .all()
        .await?;

    let count = levels.len();

    if count == 0 {
        return Ok("-2".to_string());
    }

    let response = format!(
        "{}#{}#{}#{}:{}:10#{}",
        level_string(&levels),
        creator_string(&levels),
        "",
        count,
        offset,
        generate_hash(&levels),
    );

    Ok(response)
}

fn level_string(levels: &[Level]) -> String {
    levels
        .iter()
        .map(|l| {
            let length = match l.length {
                LevelLength::Tiny => 0,
                LevelLength::Short => 1,
                LevelLength::Medium => 2,
                LevelLength::Long => 3,
                LevelLength::XL => 4,
            };

            let demon_difficulty = match l.demon_difficulty {
                DemonDifficulty::Easy => 3,
                DemonDifficulty::Medium => 4,
                DemonDifficulty::Hard => 0,
                DemonDifficulty::Insane => 5,
                DemonDifficulty::Extreme => 6,
            };

            gd_format!(
                ":",
                1 => l.id,
                2 => l.name,
                6 => l.user_id,
                10 => l.downloads,
                12 => l.official_song_id,
                14 => l.likes,
                15 => length,
                16 => l.dislikes,
                17 => u8::from(l.is_demon),
                18 => l.stars,
                25 => u8::from(l.is_auto),
                31 => u8::from(l.is_two_player),
                37 => l.coins,
                38 => u8::from(l.has_verified_coins),
                39 => l.requested_stars,
                43 => demon_difficulty,
                44 => u8::from(l.is_gauntlet),
                45 => l.objects,
                62 => l.created_at.timestamp(),
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn creator_string(levels: &[Level]) -> String {
    levels
        .iter()
        .map(|l| format!("{}:{}:{}", l.user_id, l.username, l.user_id))
        .collect::<Vec<_>>()
        .join("|")
}

fn generate_hash(levels: &[Level]) -> String {
    let mut hash_input = String::new();

    for level in levels {
        let level_id = level.id.to_string();
        let first = level_id.chars().next().unwrap_or('0');
        let last = level_id.chars().next_back().unwrap_or('0');
        let stars = level.stars.to_string();
        let verified = u8::from(level.has_verified_coins);

        let _ = write!(hash_input, "{first}{last}{stars}{verified}");
    }

    util::salt_and_sha1(&hash_input, "xI25fpAapCQg")
}
