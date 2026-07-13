use crate::Database;
use crate::error::AppError;
use cornucopia::queries::user::get_hash_and_salt;
use sha1::Digest;
use sha1::Sha1;
use sha2::Sha256;
use subtle::ConstantTimeEq;

pub async fn verify_gjp2(user_id: i32, gjp2: &str) -> crate::Result<()> {
    let client = Database::acquire().await?;
    let auth = get_hash_and_salt().bind(&client, &user_id).one().await?;

    let mut hasher = Sha256::new();
    hasher.update(gjp2);
    hasher.update(&auth.salt);
    let hash = hasher.finalize();

    if bool::from(hash.ct_eq(&auth.hash)) {
        Ok(())
    } else {
        Err(AppError)
    }
}

pub fn is_ascii_alphanumeric(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_alphanumeric())
}

pub fn salt_and_sha1(input: &str, salt: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    hasher.update(salt.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn cyclic_xor(data: &mut [u8], key: &[u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
}
