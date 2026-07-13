use crate::Database;
use crate::Result;
use crate::gd_format;
use axum_extra::extract::Form;
use cornucopia::queries::user::search_users;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    str: String,
    page: i64,
}

pub async fn getGJUsers20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    let offset = form.page * 10;

    let users = search_users()
        .bind(&client, &form.str, &form.accountID, &offset)
        .all()
        .await?;

    let count = users.len();

    if count == 0 {
        return Ok("-2".to_string());
    }

    let response = users
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

    Ok(format!("{response}#{count}:{offset}:10"))
}
