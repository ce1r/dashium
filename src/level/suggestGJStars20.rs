use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use cornucopia::queries::level::rate_level;
use cornucopia::types::Rating;
use cornucopia::types::Role;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    levelID: i32,
    stars: i16,
    feature: u8,
}

pub async fn suggestGJStars20(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    let auth = util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let rating = match form.feature {
        1 => Rating::Feature,
        2 => Rating::Epic,
        3 => Rating::Legendary,
        4 => Rating::Mythic,
        _ => Rating::Star,
    };

    if auth.role == Role::Administrator {
        rate_level()
            .bind(&client, &form.levelID, &rating, &form.stars)
            .await?;
    }

    Ok("1")
}
