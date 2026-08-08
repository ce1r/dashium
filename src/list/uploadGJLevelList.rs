use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::list::create_list;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    listName: String,
    listDesc: String,
    listLevels: String,
    difficulty: i16,
}

pub async fn uploadGJLevelList(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let description = String::from_utf8(URL_SAFE.decode(&form.listDesc)?)?;
    let levels: Vec<i32> = form
        .listLevels
        .split(',')
        .filter_map(|id| id.trim().parse().ok())
        .collect();

    let client = Database::acquire().await?;
    util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let list_id = create_list()
        .bind(
            &client,
            &form.listName,
            &description,
            &form.accountID,
            &levels,
            &form.difficulty,
        )
        .one()
        .await?;

    Ok(list_id.to_string())
}
