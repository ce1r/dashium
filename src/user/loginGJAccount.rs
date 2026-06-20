use crate::Database;
use crate::Result;
use axum_extra::extract::Form;
use cornucopia::queries::user::login_user;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    userName: String,
    gjp2: String,
    udid: String,
}

pub async fn loginGJAccount(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;

    let user_id = login_user()
        .bind(&client, &form.udid, &form.userName, &form.gjp2)
        .opt()
        .await?;

    let Some(user_id) = user_id else {
        return Ok("-11".to_string());
    };

    Ok(format!("{user_id},{user_id}"))
}
