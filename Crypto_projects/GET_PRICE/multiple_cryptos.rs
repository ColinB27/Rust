use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct CryptoPrices {
    prices: HashMap<String, HashMap<String, f64>>,
}

async fn fetch_crypto_prices<S: AsRef<str>>(url: S) -> Result<HashMap<String, f64>, reqwest::Error> {
    let url = url.as_ref();
    let response = reqwest::get(url).await?.json::<HashMap<String, HashMap<String, f64>>>().await?;
    
    // Flatten the nested HashMap
    let mut flat_prices = HashMap::new();
    for (crypto, values) in response {
        if let Some(price) = values.get("usd") {
            flat_prices.insert(crypto, *price);
        }
    }
    Ok(flat_prices)
}

#[tokio::main]
async fn main() {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=tether,usd-coin,dai,first-digital-usd&vs_currencies=usd";
    match fetch_crypto_prices(url).await {
        Ok(prices) => {
            for (crypto, price) in prices {
                println!("{}: ${}", crypto, price);
            }
        }
        Err(e) => println!("Error fetching prices: {}", e),
    }
}
