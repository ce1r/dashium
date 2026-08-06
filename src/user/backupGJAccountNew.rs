use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
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

    let key = Key::<ChaCha20Poly1305>::from(hash);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = Nonce::from([0u8; 12]);

    let ciphertext = cipher.encrypt(&nonce, form.saveData.as_ref())?;

    let path = format!("data/users/{}.user", form.accountID);
    let mut file = File::create(path).await?;
    file.write_all(&ciphertext).await?;

    Ok("1".to_string())
}
