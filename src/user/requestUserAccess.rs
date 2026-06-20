use crate::Database;
use crate::Result;
use axum_extra::extract::Form;
use cornucopia::queries::user::get_mod_level;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
}

pub async fn requestUserAccess(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;

    let mod_level = get_mod_level()
        .bind(&client, &form.accountID, &form.gjp2)
        .one()
        .await?;

    match mod_level {
        1 | 2 | 99 => Ok(mod_level.to_string()),
        _ => Ok("-1".to_string()),
    }
}
