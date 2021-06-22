use crate::syndication::{Entry, Syndication};
use async_trait::async_trait;

pub struct ValorantSyndication;

const URL_ROOT: &str = "https://playvalorant.com";
const URL_NEWS: &str = "https://playvalorant.com/ja-jp/news";

#[async_trait]
impl Syndication for ValorantSyndication {
    async fn get_latest_entry() -> reqwest::Result<Entry> {
        let doc = reqwest::get(URL_NEWS).await?.text().await?;

        let pat = easy_scraper::Pattern::new(
            // classが自動生成によるものに見え, 頻繁に変更されないかweb-archiveで確認したが, 現状公開当初から変更されていない
            r#"
            <a href="{{url}}">
                <div class="NewsCard-module--copyContainer--1R7N-">
                    <p class="copy-02 NewsCard-module--description--3sFiD">{{title}}</p>
                </div>
            </a>
            "#,
        )
        .unwrap();

        let result = pat.matches(&doc);
        let result = result
            .into_iter()
            .map(|map| {
                Entry::new(
                    URL_ROOT.to_owned() + map["url"].to_owned().as_ref(),
                    map["title"].to_owned(),
                )
            })
            .collect::<Vec<_>>();
        let result = result.first().unwrap().to_owned();

        Ok(result)
    }
}
