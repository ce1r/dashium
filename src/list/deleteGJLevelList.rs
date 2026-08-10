use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::list::delete_list;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    listID: i32,
}

pub async fn deleteGJLevelList(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    delete_list()
        .bind(&client, &form.listID, &form.accountID)
        .await?;

    Ok("1")
}
