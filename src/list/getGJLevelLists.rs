use crate::Database;
use crate::Result;
use crate::gd_format;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use chrono_humanize::HumanTime;
use cornucopia::queries::list::search_lists;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    str: String,
    page: i64,
}

pub async fn getGJLevelLists(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    let offset = form.page * 10;

    let lists = search_lists()
        .bind(&client, &form.str, &offset)
        .all()
        .await?;

    let count = lists.len();

    if count == 0 {
        return Ok("-2".to_string());
    }

    let response = lists
        .iter()
        .map(|l| {
            let levels = l
                .levels
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",");

            let created_at = HumanTime::from(l.created_at)
                .to_string()
                .replace(" ago", "");

            gd_format!(
                ":",
                1 => l.id,
                2 => l.name,
                3 => URL_SAFE.encode(&l.description),
                7 => l.difficulty,
                10 => l.downloads,
                14 => l.likes,
                19 => u8::from(l.rated),
                28 => created_at,
                29 => created_at,
                49 => l.user_id,
                51 => levels,
                55 => l.reward,
                56 => l.requirement,
            )
        })
        .collect::<Vec<_>>()
        .join("|");

    let creator_string = lists
        .iter()
        .map(|l| format!("{}:{}:{}", l.user_id, l.username, l.user_id))
        .collect::<Vec<_>>()
        .join("|");

    Ok(format!(
        "{response}#{creator_string}#{count}:{offset}:10#f5da5823d94bbe7208dd83a30ff427c7d88fdb99"
    ))
}
