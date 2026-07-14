use crate::Database;
use crate::Result;
use crate::gd_format;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use cornucopia::queries::social::get_blocked_list;
use cornucopia::queries::social::get_friend_list;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    r#type: u8,
}

pub async fn getGJUserList20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let user_list = match form.r#type {
        1 => {
            get_blocked_list()
                .bind(&client, &form.accountID)
                .all()
                .await?
        }
        _ => {
            get_friend_list()
                .bind(&client, &form.accountID)
                .all()
                .await?
        }
    };

    if user_list.is_empty() {
        return Ok("-2".to_string());
    }

    let response = user_list
        .iter()
        .map(|u| {
            gd_format!(
                ":",
                1 => u.username,
                2 => u.id,
                3 => u.stars,
                4 => u.demons,
                8 => u.creator_points,
                9 => u.icon,
                10 => u.color1,
                11 => u.color2,
                13 => u.secret_coins,
                14 => u.icon_type,
                15 => u.glow,
                16 => u.id,
                17 => u.user_coins,
                46 => u.diamonds,
                51 => u.color3,
                52 => u.moons,
            )
        })
        .collect::<Vec<_>>()
        .join("|");

    Ok(response)
}
