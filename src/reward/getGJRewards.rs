use crate::Database;
use crate::Result;
use crate::util;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use cornucopia::queries::user::get_udid;
use serde::Deserialize;

const CHEST_XOR_KEY: &[u8] = b"59182";

#[derive(Deserialize)]
pub struct Data {
    accountID: i32,
    gjp2: String,
    rewardType: u8,
    chk: String,
}

pub async fn getGJRewards(Form(form): Form<Data>) -> Result<impl IntoResponse> {
    let client = Database::acquire().await?;
    util::verify_gjp2(&client, form.accountID, &form.gjp2).await?;

    let udid = get_udid().bind(&client, &form.accountID).one().await?;

    let mut chk = URL_SAFE.decode(&form.chk[5..])?;
    util::cyclic_xor(&mut chk, CHEST_XOR_KEY);
    let chk = String::from_utf8(chk)?;

    let mut list = format!(
        "CHEST:{}:{}:{}:{}:300:3,3,1,1:3:300:3,3,1,1:3:{}",
        form.accountID, chk, udid, form.accountID, form.rewardType,
    )
    .into_bytes();

    util::cyclic_xor(&mut list, CHEST_XOR_KEY);

    let encoded = URL_SAFE.encode(&list);
    let hash = util::salt_and_sha1(&encoded, "pC26fpYaQCtg");

    Ok(format!("CHEST{encoded}|{hash}"))
}
