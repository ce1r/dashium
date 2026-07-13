use crate::Database;
use crate::Result;
use crate::gd_format;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use chrono_humanize::HumanTime;
use cornucopia::queries::social::get_messages;
use cornucopia::queries::social::get_sent_messages;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    page: i64,

    #[serde(default)]
    getSent: u8,
}

pub async fn getGJMessages20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let offset = form.page * 10;
    let get_sent = matches!(form.getSent, 1);

    let messages = if get_sent {
        get_sent_messages()
            .bind(&client, &form.accountID, &offset)
            .all()
            .await?
    } else {
        get_messages()
            .bind(&client, &form.accountID, &offset)
            .all()
            .await?
    };

    let count = messages.len();

    if count == 0 {
        return Ok("-2".to_string());
    }

    let response = messages
        .iter()
        .map(|m| {
            let user_id = if get_sent { m.target_id } else { m.user_id };
            let is_read = u8::from(m.is_read);
            let subject = URL_SAFE.encode(&m.subject);
            let created_at = HumanTime::from(m.created_at)
                .to_string()
                .replace(" ago", "");

            gd_format!(
                ":",
                1 => m.id,
                2 => user_id,
                4 => subject,
                6 => m.username,
                7 => created_at,
                8 => is_read,
                9 => form.getSent,
            )
        })
        .collect::<Vec<_>>()
        .join("|");

    Ok(format!("{response}#{count}:{offset}:10"))
}
