#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://baidu.com").await?.text().await?;
    println!("{:#?}", resp);
    Ok(())
}
