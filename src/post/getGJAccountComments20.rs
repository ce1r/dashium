use crate::Database;
use crate::Result;
use crate::gd_format;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::post::get_posts;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    page: i64,
}

pub async fn getGJAccountComments20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    let offset = form.page * 10;

    let posts = get_posts()
        .bind(&client, &form.accountID, &offset)
        .all()
        .await?;

    let count = posts.len();

    if count == 0 {
        return Ok("#0:0:10".to_string());
    }

    let response = posts
        .iter()
        .map(|p| {
            let body = URL_SAFE.encode(&p.body);

            gd_format!(
                "~",
                2 => body,
                4 => p.likes,
                6 => p.id,
                9 => p.created_at,
            )
        })
        .collect::<Vec<_>>()
        .join("|");

    Ok(format!("{response}#{count}:{offset}:10"))
}
