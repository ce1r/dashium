use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
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

pub async fn syncGJAccountNew(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    let hash: [u8; 32] = verify_gjp2(&client, form.accountID, &form.gjp2)
        .await?
        .try_into()
        .unwrap_or_default();

    let path = format!("data/users/{}.user", form.accountID);
    let save_data = fs::read(path).await?;

    let key = Key::from(hash);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = Nonce::from([0u8; 12]);

    let ciphertext = cipher.decrypt(&nonce, save_data.as_ref())?;

    Ok(format!("{};21;30;a;a", String::from_utf8(ciphertext)?))
}
