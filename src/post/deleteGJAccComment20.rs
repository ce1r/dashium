use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use cornucopia::queries::post::delete_post;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    commentID: i32,
    gjp2: String,
}

pub async fn deleteGJAccComment20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(form.accountID, &form.gjp2).await?;

    delete_post()
        .bind(&client, &form.commentID, &form.accountID)
        .await?;

    Ok("1".to_string())
}
