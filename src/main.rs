use site_recon::get;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    println!("Hello, world!");

    let res = get().await;
    println!("{:#?}", res);

    Ok(())
}
