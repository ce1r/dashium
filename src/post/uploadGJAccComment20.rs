use crate::Database;
use crate::Result;
use crate::command::Command;
use crate::command::command;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::post::create_post;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    comment: String,
    gjp2: String,
}

pub async fn uploadGJAccComment20(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let body = String::from_utf8(URL_SAFE.decode(&form.comment)?)?;

    let client = Database::acquire().await?;
    let auth = util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    if let Some(input) = body.strip_prefix('/') {
        let args = shell_words::split(input.trim()).unwrap_or_default();

        let cmd = command().run_inner(args.as_slice())?;

        return Command::execute_command(cmd, auth.role);
    }

    let comment_id = create_post()
        .bind(&client, &form.accountID, &body)
        .one()
        .await?;

    Ok(comment_id.to_string())
}
