use async_trait::async_trait;
use chrono::prelude::*;

pub mod valorant;

#[async_trait]
pub trait Syndication {
    async fn get_latest_entry() -> reqwest::Result<Entry>;
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub url: String,
    pub title: String,
    pub updated_at: Option<NaiveDateTime>,
}

impl Entry {
    pub fn new(url: String, title: String) -> Entry {
        Entry {
            url,
            title,
            updated_at: None,
        }
    }
}
