use axum_extra::extract::Form;
use cornucopia::queries::post::delete_post;
use serde::Deserialize;

use crate::Database;
use crate::Result;

#[derive(Deserialize)]
pub struct Data {
    commentID: i32,
    gjp2: String,
}

pub async fn deleteGJAccComment20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;

    delete_post()
        .bind(&client, &form.commentID, &form.gjp2)
        .await?;

    Ok("1".to_string())
}
