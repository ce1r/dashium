use crate::Database;
use crate::Result;
use crate::util::is_ascii_alphanumeric;
use crate::util::salt_and_sha1;
use axum_extra::extract::Form;
use cornucopia::queries::user::create_user;
use cornucopia::queries::user::is_email_taken;
use cornucopia::queries::user::is_username_taken;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    userName: String,
    email: String,
    password: String,
}

pub async fn registerGJAccount(Form(form): Form<Data>) -> Result<String> {
    if form.userName.len() > 20 {
        return Ok("-4".to_string());
    }

    if !is_ascii_alphanumeric(&form.userName) {
        return Ok("-4".to_string());
    }

    if form.userName.len() < 3 {
        return Ok("-9".to_string());
    }

    if form.password.len() < 8 {
        return Ok("-8".to_string());
    }

    if !is_ascii_alphanumeric(&form.password) {
        return Ok("-5".to_string());
    }

    let client = Database::acquire().await?;

    let username_taken = is_username_taken()
        .bind(&client, &form.userName)
        .one()
        .await?;

    let email_taken = is_email_taken().bind(&client, &form.email).one().await?;

    if username_taken {
        return Ok("-2".to_string());
    }

    if email_taken {
        return Ok("-3".to_string());
    }

    let gjp2 = salt_and_sha1(&form.password, "mI29fmAnxgTs");

    create_user()
        .bind(&client, &form.userName, &form.email, &gjp2)
        .await?;

    Ok("1".to_string())
}
