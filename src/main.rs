use reqwest;
use tokio;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print!("Getting IP Address: ");
    let ip = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?;
    println!("{}", ip);

    Ok(())
}
