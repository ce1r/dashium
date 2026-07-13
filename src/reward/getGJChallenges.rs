use crate::Database;
use crate::Result;
use crate::util::cyclic_xor;
use crate::util::salt_and_sha1;
use crate::util::seconds_until_midnight;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::reward::get_quests;
use cornucopia::types::ItemType;
use serde::Deserialize;

const QUEST_XOR_KEY: &[u8] = b"19847";

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    chk: String,
    udid: String,
}

pub async fn getGJChallenges(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;
    let mut chk = URL_SAFE.decode(&form.chk[5..])?;
    cyclic_xor(&mut chk, QUEST_XOR_KEY);
    let chk = String::from_utf8(chk)?;

    let quests = get_quests().bind(&client).all().await?;

    let quest_string = quests
        .iter()
        .map(|q| {
            let item_type = match q.item_type {
                ItemType::orbs => 1,
                ItemType::coins => 2,
                ItemType::stars => 3,
            };

            format!(
                "{},{},{},{},{}",
                q.id, item_type, q.amount, q.reward, q.name
            )
        })
        .collect::<Vec<_>>()
        .join(":");

    let mut list = format!(
        "QUEST:{}:{}:{}:{}:{}:{}",
        form.accountID,
        chk,
        form.udid,
        form.accountID,
        seconds_until_midnight(),
        quest_string,
    )
    .into_bytes();

    cyclic_xor(&mut list, QUEST_XOR_KEY);

    let encoded = URL_SAFE.encode(list);
    let hash = salt_and_sha1(&encoded, "oC36fpYaPtdg");

    Ok(format!("QUEST{encoded}|{hash}"))
}
