use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::types::Role;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
}

pub async fn requestUserAccess(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    let auth = util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    match auth.role {
        Role::User => Ok("-1"),
        Role::Moderator => Ok("1"),
        Role::ElderModerator | Role::Administrator => Ok("2"),
        Role::LeaderboardModerator => Ok("99"),
    }
}
