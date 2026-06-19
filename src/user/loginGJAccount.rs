use crate::Database;
use crate::Result;
use axum_extra::extract::Form;
use cornucopia::queries::user::verify_gjp2;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    userName: String,
    gjp2: String,
}

pub async fn loginGJAccount(Form(form): Form<Data>) -> Result<String> {
    let client = &Database::acquire().await?;

    let user_id = verify_gjp2()
        .bind(client, &form.userName, &form.gjp2)
        .opt()
        .await?;

    let Some(user_id) = user_id else {
        return Ok("-11".to_string());
    };

    Ok(format!("{user_id},{user_id}"))
}
