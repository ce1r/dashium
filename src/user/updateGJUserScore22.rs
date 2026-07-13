use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use cornucopia::queries::user::save_stats;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    stars: i32,
    demons: i32,
    diamonds: i32,
    moons: i32,
    coins: i32,
    userCoins: i32,
    accIcon: i16,
    accShip: i16,
    accBall: i16,
    accBird: i16,
    accDart: i16,
    accRobot: i16,
    accSpider: i16,
    accSwing: i16,
    accJetpack: i16,
    accGlow: i16,
    accExplosion: i16,
    icon: i16,
    iconType: i16,
    color1: i16,
    color2: i16,
    color3: i16,
}

pub async fn updateGJUserScore22(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(form.accountID, &form.gjp2).await?;

    let user_id = save_stats()
        .bind(
            &client,
            &form.stars,
            &form.demons,
            &form.diamonds,
            &form.moons,
            &form.coins,
            &form.userCoins,
            &form.accIcon,
            &form.accShip,
            &form.accBall,
            &form.accBird,
            &form.accDart,
            &form.accRobot,
            &form.accSpider,
            &form.accSwing,
            &form.accJetpack,
            &form.accGlow,
            &form.accExplosion,
            &form.icon,
            &form.iconType,
            &form.color1,
            &form.color2,
            &form.color3,
            &form.accountID,
        )
        .one()
        .await?;

    Ok(user_id.to_string())
}
