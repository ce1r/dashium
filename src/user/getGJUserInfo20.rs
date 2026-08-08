use crate::Database;
use crate::Result;
use crate::gd_format;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::user::get_user_by_id;
use cornucopia::types::CommentSetting;
use cornucopia::types::MessageSetting;
use cornucopia::types::Role;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    targetAccountID: i32,
}

pub async fn getGJUserInfo20(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let user = get_user_by_id()
        .bind(&client, &form.targetAccountID)
        .one()
        .await?;

    let mod_level = match user.role {
        Role::User => 0,
        Role::Moderator => 1,
        Role::ElderModerator | Role::Administrator => 2,
        Role::LeaderboardModerator => 3,
    };

    let message_setting = match user.message_setting {
        MessageSetting::All => 0,
        MessageSetting::FriendsOnly => 1,
        MessageSetting::None => 2,
    };

    let comment_setting = match user.comment_setting {
        CommentSetting::All => 0,
        CommentSetting::FriendsOnly => 1,
        CommentSetting::None => 2,
    };

    let response = gd_format!(
        ":",
        1 => user.username,
        2 => user.id,
        3 => user.stars,
        4 => user.demons,
        6 => user.star_rank,
        8 => user.creator_points,
        10 => user.color1,
        11 => user.color2,
        13 => user.secret_coins,
        16 => user.id,
        17 => user.user_coins,
        18 => message_setting,
        19 => u8::from(user.accept_friend_requests),
        20 => user.youtube,
        21 => user.cube,
        22 => user.ship,
        23 => user.ball,
        24 => user.ufo,
        25 => user.wave,
        26 => user.robot,
        28 => user.glow,
        29 => 1,
        30 => user.star_rank,
        43 => user.spider,
        44 => user.twitter,
        45 => user.twitch,
        46 => user.diamonds,
        48 => user.explosion,
        49 => mod_level,
        50 => comment_setting,
        51 => user.color3,
        52 => user.moons,
        53 => user.swing,
        54 => user.jetpack,
        58 => user.discord,
        59 => user.instagram,
        60 => user.tiktok,
    );

    Ok(response)
}
