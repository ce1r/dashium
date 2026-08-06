use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use cornucopia::queries::user::get_mod_level;
use cornucopia::types::ModLevel;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
}

pub async fn requestUserAccess(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let mod_level = get_mod_level().bind(&client, &form.accountID).one().await?;

    match mod_level {
        ModLevel::None => Ok("-1".to_string()),
        ModLevel::Moderator => Ok("1".to_string()),
        ModLevel::ElderModerator => Ok("2".to_string()),
        ModLevel::LeaderboardModerator => Ok("99".to_string()),
    }
}
