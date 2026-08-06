use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use chacha20poly1305::ChaCha20Poly1305;
use chacha20poly1305::KeyInit;
use chacha20poly1305::Nonce;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::aead::Key;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    saveData: String,
}

pub async fn backupGJAccountNew(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    let hash: [u8; 32] = verify_gjp2(&client, form.accountID, &form.gjp2)
        .await?
        .try_into()
        .unwrap_or_default();

    let (user_data, user_levels) = form
        .saveData
        .split_once(';')
        .ok_or_else(|| anyhow::anyhow!(""))?;

    let user_data = URL_SAFE.decode(user_data)?;
    let user_levels = URL_SAFE.decode(user_levels)?;

    let key = Key::<ChaCha20Poly1305>::from(hash);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = Nonce::from([0u8; 12]);

    let ciphertext = cipher.encrypt(&nonce, user_data.as_ref())?;

    let path1 = format!("data/users/{}.userdata", form.accountID);
    let path2 = format!("data/users/{}.userlevels", form.accountID);
    let mut file1 = File::create(path1).await?;
    let mut file2 = File::create(path2).await?;
    file1.write_all(&ciphertext).await?;
    file2.write_all(&user_levels).await?;

    Ok("1".to_string())
}
