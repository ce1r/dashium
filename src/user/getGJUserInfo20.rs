use crate::Database;
use crate::Result;
use crate::gd_format;
use axum_extra::extract::Form;
use cornucopia::queries::user::get_user;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    targetAccountID: i32,
}

pub async fn getGJUserInfo20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;

    let user = get_user()
        .bind(&client, &form.targetAccountID)
        .one()
        .await?;

    let response = gd_format!(
        1 => user.username,
        2 => user.id,
        3 => user.stars,
        4 => user.demons,
        8 => user.creator_points,
        10 => user.color1,
        11 => user.color2,
        13 => user.secret_coins,
        16 => user.id,
        17 => user.user_coins,
        18 => user.message_setting,
        19 => user.friend_setting,
        20 => user.youtube,
        21 => user.cube,
        22 => user.ship,
        23 => user.ball,
        24 => user.ufo,
        25 => user.wave,
        26 => user.robot,
        28 => user.glow,
        29 => u8::from(user.is_activated),
        43 => user.spider,
        44 => user.twitter,
        45 => user.twitch,
        46 => user.diamonds,
        48 => user.explosion,
        49 => user.mod_level,
        50 => user.comment_setting,
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
