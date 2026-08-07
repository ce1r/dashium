use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::user::get_mod_level;
use cornucopia::types::ModLevel;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
}

pub async fn requestUserAccess(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let mod_level = get_mod_level().bind(&client, &form.accountID).one().await?;

    match mod_level {
        ModLevel::None => Ok("-1"),
        ModLevel::Moderator => Ok("1"),
        ModLevel::ElderModerator => Ok("2"),
        ModLevel::LeaderboardModerator => Ok("99"),
    }
}
