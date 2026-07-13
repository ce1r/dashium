use crate::Database;
use crate::Result;
use axum_extra::extract::Form;
use cornucopia::queries::user::login_user;
use serde::Deserialize;
use sha2::Digest;
use sha2::Sha256;
use subtle::ConstantTimeEq;

#[derive(Deserialize)]
pub struct Data {
    userName: String,
    gjp2: String,
    udid: String,
}

pub async fn loginGJAccount(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;

    let auth = login_user()
        .bind(&client, &form.udid, &form.userName)
        .opt()
        .await?;

    let Some(auth) = auth else {
        return Ok("-11".to_string());
    };

    let mut hasher = Sha256::new();
    hasher.update(&form.gjp2);
    hasher.update(&auth.salt);
    let hash = hasher.finalize();

    if !bool::from(hash.ct_eq(&auth.hash)) {
        return Ok("-11".to_string());
    }

    let user_id = auth.id;

    Ok(format!("{user_id},{user_id}"))
}
