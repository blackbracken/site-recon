use crate::syndication::*;
use reqwest::Result;

pub(crate) mod syndication;

pub async fn get() -> Result<Entry> {
    let x = crate::syndication::valorant::ValorantSyndication::get_latest_entry().await?;

    Ok(x)
}
