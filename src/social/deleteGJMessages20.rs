use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use cornucopia::queries::social::delete_message;
use cornucopia::queries::social::delete_messages;
use cornucopia::queries::social::delete_sent_message;
use cornucopia::queries::social::delete_sent_messages;
use serde::Deserialize;
use serde_with::BoolFromInt;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,

    #[serde(default)]
    messages: String,

    #[serde(default)]
    messageID: i32,

    #[serde_as(as = "BoolFromInt")]
    #[serde(default)]
    isSender: bool,
}

pub async fn deleteGJMessages20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    if form.messages.is_empty() {
        if form.isSender {
            delete_sent_message()
                .bind(&client, &form.messageID, &form.accountID)
                .await?;
        } else {
            delete_message()
                .bind(&client, &form.messageID, &form.accountID)
                .await?;
        }
    } else {
        let message_ids: Vec<i32> = form
            .messages
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if form.isSender {
            delete_sent_messages()
                .bind(&client, &form.messageID, &message_ids)
                .await?;
        } else {
            delete_messages()
                .bind(&client, &form.messageID, &message_ids)
                .await?;
        }
    }

    Ok("1".to_string())
}
