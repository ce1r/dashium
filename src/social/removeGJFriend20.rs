use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::social::remove_friend;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    targetAccountID: i32,
}

pub async fn removeGJFriend20(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    remove_friend()
        .bind(&client, &form.accountID, &form.targetAccountID)
        .await?;

    Ok("1")
}
