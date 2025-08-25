use reqwest;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

#[derive(Deserialize)]
struct TetherPrice {
    tether: TetherData,
}

#[derive(Deserialize)]
struct TetherData {
    usd: f64,
}

async fn fetch_tether_price() -> Result<f64, reqwest::Error> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=tether&vs_currencies=usd";
    let response = reqwest::get(url).await?.json::<TetherPrice>().await?;
    Ok(response.tether.usd)
}
async fn fetch_crypto_price<S: AsRef<str>>(url: S) -> Result<f64, reqwest::Error> {
    let url = url.as_ref();
    let response = reqwest::get(url).await?.json::<TetherPrice>().await?;
    Ok(response.tether.usd)
}

#[tokio::main]
async fn main() {
    loop {
        match fetch_tether_price().await {
            Ok(price) => println!("Current Tether (USDT) price: ${}", price),
            Err(e) => eprintln!("Error fetching Tether price: {}", e),
        }
        sleep(Duration::from_secs(60)).await; // Update every 60 seconds
    }
}
