use crate::Database;
use crate::Result;
use crate::gd_format;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::user::get_leaderboard;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    r#type: String,
    stat: i16,
}

pub async fn getGJScores20(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;

    let stat = if form.r#type == "creators" {
        4
    } else {
        form.stat
    };

    let users = get_leaderboard().bind(&client, &stat).all().await?;

    let response = users
        .iter()
        .map(|u| {
            gd_format!(
                ":",
                1 => u.username,
                2 => u.id,
                3 => u.stars,
                4 => u.demons,
                6 => u.star_rank,
                8 => u.creator_points,
                9 => u.icon,
                10 => u.color1,
                11 => u.color2,
                13 => u.secret_coins,
                14 => u.icon_type,
                15 => u.glow,
                16 => u.id,
                17 => u.user_coins,
                30 => u.star_rank,
                46 => u.diamonds,
                51 => u.color3,
                52 => u.moons,
            )
        })
        .collect::<Vec<_>>()
        .join("|");

    Ok(response)
}
