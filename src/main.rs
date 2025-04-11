pub mod rust_file_master;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let body = reqwest::get("https://web.snapchat.com")
    .await?
    .text()
    .await?;

    println!("body = {body:?}");
    Ok(())
}
