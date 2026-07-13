use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use cornucopia::queries::user::load_data;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
}

pub async fn syncGJAccountNew(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(form.accountID, &form.gjp2).await?;

    let save_data = load_data().bind(&client, &form.accountID).one().await?;

    Ok(format!("{save_data};21;30;a;a"))
}
