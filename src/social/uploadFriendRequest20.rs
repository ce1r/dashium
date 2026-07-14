use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::social::create_friend_request;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    comment: String,
    gjp2: String,
    toAccountID: i32,
}

pub async fn uploadFriendRequest20(Form(form): Form<Data>) -> Result<String> {
    let body = String::from_utf8(URL_SAFE.decode(&form.comment)?)?;

    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    create_friend_request()
        .bind(&client, &form.accountID, &form.toAccountID, &body)
        .await?;

    Ok("1".to_string())
}
