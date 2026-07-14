use crate::Database;
use crate::Result;
use crate::util::verify_gjp2;
use axum_extra::extract::Form;
use cornucopia::queries::social::delete_friend_requests;
use serde::Deserialize;
use serde_with::BoolFromInt;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    targetAccountID: i32,

    #[serde(default)]
    accounts: String,

    #[serde_as(as = "BoolFromInt")]
    #[serde(default)]
    isSender: bool,
}

pub async fn deleteGJFriendRequests20(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let (user_id, target_id) = if form.isSender {
        (form.accountID, form.targetAccountID)
    } else {
        (form.targetAccountID, form.accountID)
    };

    if form.accounts.is_empty() {
        delete_friend_requests()
            .bind(&client, &user_id, &vec![target_id])
            .await?;
    } else {
        let target_ids = form
            .accounts
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect::<Vec<i32>>();

        delete_friend_requests()
            .bind(&client, &user_id, &target_ids)
            .await?;
    }

    Ok("1".to_string())
}
