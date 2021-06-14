use reqwest::Result;

#[derive(Debug)]
struct ValorantNews {
    title: String,
    url: String,
}

const URL_ROOT: &str = "https://playvalorant.com";
const URL_NEWS: &str = "https://playvalorant.com/ja-jp/news";

fn get_news() -> Result<Vec<ValorantNews>> {
    let doc = reqwest::blocking::get(URL_NEWS)?.text()?;

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

    Ok(result
        .into_iter()
        .map(|m| ValorantNews {
            title: m["title"].to_owned(),
            url: URL_ROOT.to_owned() + m["url"].to_owned().as_ref(),
        })
        .collect())
}

fn main() {
    println!("Hello, world!");

    let res = get_news();
    println!("{:#?}", res);
}
