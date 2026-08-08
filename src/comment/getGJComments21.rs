use crate::Database;
use crate::Result;
use crate::gd_format;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use chrono_humanize::HumanTime;
use cornucopia::queries::comment::get_comments_by_date;
use cornucopia::queries::comment::get_comments_by_likes;
use cornucopia::types::Role;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    levelID: i32,
    page: i64,

    #[serde(default)]
    mode: u8,
}

pub async fn getGJComments21(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    let offset = form.page * 10;

    let comments = match form.mode {
        1 => {
            get_comments_by_likes()
                .bind(&client, &form.levelID, &offset)
                .all()
                .await?
        }
        _ => {
            get_comments_by_date()
                .bind(&client, &form.levelID, &offset)
                .all()
                .await?
        }
    };

    let count = comments.len();

    if count == 0 {
        return Ok("-2".to_string());
    }

    let response = comments
        .iter()
        .map(|c| {
            let created_at = HumanTime::from(c.created_at)
                .to_string()
                .replace(" ago", "");
            let mod_level = match c.role {
                Role::User => 0,
                Role::Moderator => 1,
                Role::ElderModerator | Role::Administrator => 2,
                Role::LeaderboardModerator => 3,
            };

            let comment = gd_format!(
                "~",
                2 => URL_SAFE.encode(&c.body),
                3 => c.user_id,
                4 => c.likes,
                6 => c.id,
                7 => u8::from(c.is_spam),
                8 => c.user_id,
                9 => created_at,
                10 => c.percent,
                11 => mod_level,
                12 => c.chat_color,
            );

            let user = gd_format!(
                "~",
                1 => c.username,
                9 => c.icon,
                10 => c.color1,
                11 => c.color2,
                14 => c.icon_type,
                15 => c.glow,
                16 => c.user_id,
                51 => c.color3,
            );

            format!("{comment}:{user}")
        })
        .collect::<Vec<_>>()
        .join("|");

    Ok(format!("{response}#{count}:{offset}:10"))
}
