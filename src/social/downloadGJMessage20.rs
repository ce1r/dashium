use crate::Database;
use crate::Result;
use crate::gd_format;
use crate::util::cyclic_xor;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use chrono_humanize::HumanTime;
use cornucopia::queries::social::download_message;
use serde::Deserialize;

const MESSAGE_XOR_KEY: &[u8] = b"14251";

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    messageID: i32,
}

pub async fn downloadGJMessage20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let message = download_message()
        .bind(&client, &form.messageID, &form.accountID)
        .one()
        .await?;

    let subject = URL_SAFE.encode(&message.subject);
    let is_read = u8::from(message.is_read);
    let created_at = HumanTime::from(message.created_at)
        .to_string()
        .replace(" ago", "");

    let mut body = message.body.into_bytes();
    cyclic_xor(&mut body, MESSAGE_XOR_KEY);
    let body = URL_SAFE.encode(body);

    let response = gd_format!(
        ":",
        1 => message.id,
        2 => message.user_id,
        4 => subject,
        5 => body,
        6 => message.username,
        7 => created_at,
        8 => is_read,
    );

    Ok(response)
}
