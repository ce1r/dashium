use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use chacha20poly1305::ChaCha20Poly1305;
use chacha20poly1305::Key;
use chacha20poly1305::KeyInit;
use chacha20poly1305::Nonce;
use chacha20poly1305::aead::Aead;
use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
}

pub async fn syncGJAccountNew(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    let hash: [u8; 32] = util::verify_gjp2(&client, form.accountID, &form.gjp2)
        .await?
        .try_into()
        .unwrap_or_default();

    let path1 = format!("data/users/{}.userdata", form.accountID);
    let path2 = format!("data/users/{}.userlevels", form.accountID);
    let user_data = fs::read(path1).await?;
    let user_levels = fs::read(path2).await?;

    let key = Key::from(hash);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = Nonce::from([0u8; 12]);

    let ciphertext = cipher.decrypt(&nonce, user_data.as_ref())?;

    Ok(format!(
        "{};{};21;30;a;a",
        URL_SAFE.encode(ciphertext),
        URL_SAFE.encode(user_levels)
    ))
}
