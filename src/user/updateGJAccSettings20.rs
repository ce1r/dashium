use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::user::update_settings;
use cornucopia::types::CommentSetting;
use cornucopia::types::MessageSetting;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    mS: i16,
    frS: i16,
    cS: i16,
    yt: String,
    twitter: String,
    twitch: String,
    discord: String,
    instagram: String,
    tiktok: String,
}

pub async fn updateGJAccSettings20(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let accept_friend_requests = form.frS == 0;
    let message_setting = match form.mS {
        1 => MessageSetting::FriendsOnly,
        2 => MessageSetting::None,
        _ => MessageSetting::All,
    };

    let comment_setting = match form.cS {
        1 => CommentSetting::FriendsOnly,
        2 => CommentSetting::None,
        _ => CommentSetting::All,
    };

    update_settings()
        .bind(
            &client,
            &accept_friend_requests,
            &message_setting,
            &comment_setting,
            &form.yt,
            &form.twitter,
            &form.twitch,
            &form.discord,
            &form.instagram,
            &form.tiktok,
            &form.accountID,
        )
        .await?;

    Ok("1")
}
