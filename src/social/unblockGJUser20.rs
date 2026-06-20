use crate::Database;
use crate::Result;
use axum_extra::extract::Form;
use cornucopia::queries::social::unblock_user;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    targetAccountID: i32,
}

pub async fn unblockGJUser20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;

    unblock_user()
        .bind(&client, &form.accountID, &form.targetAccountID, &form.gjp2)
        .await?;

    Ok("1".to_string())
}
