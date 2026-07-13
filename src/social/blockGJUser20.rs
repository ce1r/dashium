use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use cornucopia::queries::social::block_user;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    targetAccountID: i32,
}

pub async fn blockGJUser20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    block_user()
        .bind(&client, &form.accountID, &form.targetAccountID)
        .await?;

    Ok("1".to_string())
}
