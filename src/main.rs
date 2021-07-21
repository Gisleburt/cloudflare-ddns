use reqwest;
use serde::Serialize;
use serde_json::json;
use tokio;

use std::error::Error;
use std::env;

const CLOUDFLARE_API_BASE: &str = "https://api.cloudflare.com/client/v4";

#[derive(Serialize)]
struct Record {
    #[serde(rename="type")]
    _type: String,
    name: String,
    content: String,
    ttl: u32,
    proxied: bool,
}

fn get_env_or_panic(var: &str) -> String {
    // ToDo: Make this a build time check
    if var.contains('=') || var.contains('\0') {
        panic!("environment variable name must not containt an ASCII equals sign '=' or the NUL character '\0'",)
    }
    match env::var(var) {
        Ok(val) => val,
        Err(_) => panic!("environment variable '{}' was not set or was invalid", var),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let zone_id = get_env_or_panic("CLOUDFLARE_ZONE_ID");
    let record_id = get_env_or_panic("CLOUDFLARE_RECORD_ID");
    let api_token = get_env_or_panic("CLOUDFLARE_API_TOKEN");
    let email = get_env_or_panic("CLOUDFLARE_EMAIL");
    let key = get_env_or_panic("CLOUDFLARE_API_KEY");
    let record_name = get_env_or_panic("RECORD_NAME");

    print!("Getting IP Address: ");
    let ip = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?;
    println!("{}", ip);

    let record = Record {
        _type: "A".to_string(),
        name: record_name,
        content: ip,
        ttl: 1,
        proxied: false
    };

    println!("Updating Record");
    let url = format!("{}/zones/{}/dns_records/{}", CLOUDFLARE_API_BASE, zone_id, record_id);
    let response = reqwest::Client::new()
        .put(url)
        // .header("Authorization", format!("Bearer {}", api_token))
        .header("X-Auth-Email", email)
        .header("X-Auth-Key", key)
        .header("Content-Type", "application/json")
        .body(json!(record).to_string())
        .send()
        .await?;

    match response.status().as_u16() {
        200..=299 => println!("Success"),
        _ => {
            println!("Error");
            println!("{}", response.status());
            println!("{}", response.text().await?);
        }
    }

    Ok(())
}
