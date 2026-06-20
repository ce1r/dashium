use crate::Database;
use crate::Result;
use crate::util::cyclic_xor;
use crate::util::salt_and_sha1;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::reward::get_udid;
use serde::Deserialize;

const CHEST_XOR_KEY: &[u8] = b"59182";

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    rewardType: u8,
    chk: String,
}

pub async fn getGJRewards(Form(form): Form<Data>) -> Result<String> {
    let client = Database::acquire().await?;

    let udid = get_udid()
        .bind(&client, &form.accountID, &form.gjp2)
        .one()
        .await?;

    let mut chk = URL_SAFE.decode(&form.chk[5..])?;
    cyclic_xor(&mut chk, CHEST_XOR_KEY);

    let chk = String::from_utf8(chk)?;

    let mut list = format!(
        "CHEST:{}:{}:{}:{}:300:3,3,1,1:3:300:3,3,1,1:3:{}",
        form.accountID, chk, udid, form.accountID, form.rewardType,
    )
    .into_bytes();

    cyclic_xor(&mut list, CHEST_XOR_KEY);

    let encoded = URL_SAFE.encode(&list);
    let hash = salt_and_sha1(&encoded, "pC26fpYaQCtg");

    Ok(format!("CHEST{encoded}|{hash}"))
}
