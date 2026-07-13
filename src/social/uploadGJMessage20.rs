use crate::Database;
use crate::Result;
use crate::util::cyclic_xor;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::social::create_message;
use serde::Deserialize;

const MESSAGE_XOR_KEY: &[u8] = b"14251";

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    toAccountID: i32,
    subject: String,
    body: String,
}

pub async fn uploadGJMessage20(Form(form): Form<Data>) -> Result<String> {
    let subject = String::from_utf8(URL_SAFE.decode(&form.subject)?)?;
    let mut body = URL_SAFE.decode(&form.body)?;
    cyclic_xor(&mut body, MESSAGE_XOR_KEY);
    let body = String::from_utf8(body)?;

    let client = Database::acquire().await?;
    verify_gjp2(form.accountID, &form.gjp2).await?;

    create_message()
        .bind(&client, &form.accountID, &form.toAccountID, &subject, &body)
        .await?;

    Ok("1".to_string())
}
