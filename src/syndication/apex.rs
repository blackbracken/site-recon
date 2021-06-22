use crate::syndication::{Entry, Syndication};
use async_trait::async_trait;

pub struct ApexSyndication;

const URL_NEWS: &str = "https://www.ea.com/ja-jp/games/apex-legends/news#game-updates";

#[async_trait]
impl Syndication for ApexSyndication {
    async fn get_latest_entry() -> reqwest::Result<Entry> {
        let doc = reqwest::get(URL_NEWS).await?.text().await?;

        let pat = easy_scraper::Pattern::new(
            r#"
            <ea-container slot="container" filter-key="All">
                <ea-tile title-text="{{title}}">
                    <ea-tile-copy slot="copy">{{description}}</ea-tile-copy>
                    <ea-cta link-url="{{url}}"></ea-cta>
                </ea-tile>
            </ea-container>
            "#,
        )
        .unwrap();

        let result = pat.matches(&doc);
        let result = result
            .into_iter()
            .map(|map| {
                let title = map["title"].to_owned();
                let description = map["description"].to_owned();

                if title.contains("パッチノート") || description.contains("アップデート")
                {
                    Some(Entry::new(map["url"].to_owned(), title))
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        let result = result.first().unwrap().to_owned();

        Ok(result)
    }
}
