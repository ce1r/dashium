use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::user::create_user;
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

pub async fn registerGJAccount(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    util::is_valid_username(&form.userName)?;
    util::is_valid_password(&form.password)?;

    let gjp2 = util::salt_and_sha1(&form.password, "mI29fmAnxgTs");

    let mut salt = [0u8; 16];
    rand::rng().try_fill_bytes(&mut salt);

    let mut hasher = Sha256::new();
    hasher.update(gjp2);
    hasher.update(salt);
    let hash = hasher.finalize();

    create_user()
        .bind(
            &client,
            &form.userName,
            &form.email,
            &hash.to_vec(),
            &salt.to_vec(),
        )
        .await?;

    Ok("1")
}
