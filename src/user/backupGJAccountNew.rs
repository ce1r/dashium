use crate::Database;
use crate::Result;
use axum_extra::extract::Form;
use cornucopia::queries::user::save_data;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    saveData: String,
}

pub async fn backupGJAccountNew(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;

    save_data()
        .bind(&client, &form.saveData, &form.accountID, &form.gjp2)
        .await?;

    Ok("1".to_string())
}
