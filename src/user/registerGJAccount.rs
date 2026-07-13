use crate::Database;
use crate::Result;
use crate::util::is_ascii_alphanumeric;
use crate::util::salt_and_sha1;
use axum_extra::extract::Form;
use cornucopia::queries::user::create_user;
use cornucopia::tokio_postgres::error::SqlState;
use rand::TryRng;
use serde::Deserialize;
use sha2::Digest;
use sha2::Sha256;

#[derive(Deserialize)]
pub struct Data {
    userName: String,
    email: String,
    password: String,
}

pub async fn registerGJAccount(Form(form): Form<Data>) -> Result<String> {
    if form.userName.len() > 20 {
        return Ok("-4".to_string());
    }

    if !is_ascii_alphanumeric(&form.userName) {
        return Ok("-4".to_string());
    }

    if form.userName.len() < 3 {
        return Ok("-9".to_string());
    }

    if form.password.len() < 8 {
        return Ok("-8".to_string());
    }

    if !is_ascii_alphanumeric(&form.password) {
        return Ok("-5".to_string());
    }

    let client = Database::acquire().await?;

    let gjp2 = salt_and_sha1(&form.password, "mI29fmAnxgTs");

    let mut salt = [0u8; 16];
    rand::rng().try_fill_bytes(&mut salt);

    let mut hasher = Sha256::new();
    hasher.update(gjp2);
    hasher.update(salt);
    let hash = hasher.finalize();

    let result = create_user()
        .bind(
            &client,
            &form.userName,
            &form.email,
            &hash.to_vec(),
            &salt.to_vec(),
        )
        .await;

    match result {
        Ok(_) => Ok("1".to_string()),
        Err(e) => {
            let Some(db_err) = e.as_db_error() else {
                return Ok("-1".to_string());
            };

            if db_err.code() != &SqlState::UNIQUE_VIOLATION {
                return Ok("-1".to_string());
            }

            Ok(match db_err.constraint() {
                Some("unique_username") => "-2",
                Some("unique_email") => "-3",
                _ => "-1",
            }
            .to_string())
        }
    }
}
