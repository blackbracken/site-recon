use site_recon::get_news;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    println!("Hello, world!");

    let res = get_news().await;
    println!("{:#?}", res);

    Ok(())
}