use crate::Database;
use crate::Result;
use crate::gd_format;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use chrono_humanize::HumanTime;
use cornucopia::queries::social::get_friend_requests;
use cornucopia::queries::social::get_sent_friend_requests;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    page: i64,

    #[serde(default)]
    getSent: u8,
}

pub async fn getGJFriendRequests20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let offset = form.page * 10;
    let get_sent = matches!(form.getSent, 1);

    let friend_requests = if get_sent {
        get_sent_friend_requests()
            .bind(&client, &form.accountID, &offset)
            .all()
            .await?
    } else {
        get_friend_requests()
            .bind(&client, &form.accountID, &offset)
            .all()
            .await?
    };

    let count = friend_requests.len();

    if count == 0 {
        return Ok("-2".to_string());
    }

    let response = friend_requests
        .iter()
        .map(|fr| {
            let user_id = if get_sent { fr.target_id } else { fr.user_id };
            let body = URL_SAFE.encode(&fr.body);
            let is_new = u8::from(fr.is_new);
            let created_at = HumanTime::from(fr.created_at)
                .to_string()
                .replace(" ago", "");

            gd_format!(
                ":",
                1 => fr.username,
                2 => user_id,
                9 => fr.icon,
                10 => fr.color1,
                11 => fr.color2,
                14 => fr.icon_type,
                15 => fr.glow,
                16 => user_id,
                32 => fr.id,
                35 => body,
                37 => created_at,
                41 => is_new,
            )
        })
        .collect::<Vec<_>>()
        .join("|");

    Ok(format!("{response}#{count}:{offset}:20"))
}
