use axum_extra::extract::Form;
use cornucopia::queries::user::create_user;
use cornucopia::tokio_postgres::error::SqlState;
use serde::Deserialize;

use crate::Database;
use crate::Result;
use crate::util::is_ascii_alphanumeric;
use crate::util::salt_and_sha1;

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

    let gjp2 = salt_and_sha1(&form.password, "mI29fmAnxgTs");

    let result = create_user()
        .bind(&client, &form.userName, &form.email, &gjp2)
        .await;

    match result {
        Ok(_) => Ok("1".to_string()),
        Err(e) => {
            if let Some(db_err) = e.as_db_error() {
                match db_err.code() {
                    &SqlState::UNIQUE_VIOLATION => {
                        if let Some(constraint) = db_err.constraint() {
                            match constraint {
                                "unique_username" => Ok("-2".to_string()),
                                "unique_email" => Ok("-3".to_string()),
                                _ => Ok("-1".to_string()),
                            }
                        } else {
                            Ok("-1".to_string())
                        }
                    }
                    _ => Ok("-1".to_string()),
                }
            } else {
                Ok("-1".to_string())
            }
        }
    }
}
