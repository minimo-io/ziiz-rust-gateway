use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct ExchangeRate {
    data: String,
    valor: String,
}

async fn fetch_brl_usd_exchange_rate() -> Result<ExchangeRate, Error> {
    let url = "https://api.bcb.gov.br/dados/serie/bcdata.sgs.10813/dados?formato=json";
    let response = reqwest::get(url).await?.json::<Vec<ExchangeRate>>().await?;

    // Get the latest exchange rate (last element in the array)
    let latest_rate = response.last().unwrap().clone();

    Ok(latest_rate)
}

#[tokio::main]
async fn main() {
    match fetch_brl_usd_exchange_rate().await {
        Ok(rate) => println!("Date: {}, BRL/USD Rate: {}", rate.data, rate.valor),
        Err(err) => eprintln!("Error fetching exchange rate: {}", err),
    }
}

// Next: let's implement redis caching (if available) before quering (not BCB, next the database)

// use reqwest::Error;
// use serde::Deserialize;
// use redis::AsyncCommands; // Import necessary traits for Redis

// #[derive(Deserialize, Debug, Clone)]
// struct ExchangeRate {
//     data: String,
//     valor: String,
// }

// async fn fetch_brl_usd_exchange_rate() -> Result<ExchangeRate, Error> {
//     let url = "https://api.bcb.gov.br/dados/serie/bcdata.sgs.10813/dados?formato=json";
//     let response = reqwest::get(url).await?.json::<Vec<ExchangeRate>>().await?;
    
//     // Get the latest exchange rate (last element in the array)
//     let latest_rate = response.last().unwrap().clone();
    
//     Ok(latest_rate)
// }

// async fn get_exchange_rate_from_redis(redis_client: &redis::Client) -> Option<ExchangeRate> {
//     // Try to connect to Redis and retrieve the exchange rate
//     if let Ok(mut con) = redis_client.get_async_connection().await {
//         if let Ok(rate) = con.get::<&str, ExchangeRate>("latest_brl_usd").await {
//             return Some(rate);
//         }
//     }
//     None // Return None if any error occurs
// }

// async fn set_exchange_rate_to_redis(redis_client: &redis::Client, rate: &ExchangeRate) -> redis::RedisResult<()> {
//     if let Ok(mut con) = redis_client.get_async_connection().await {
//         let _: () = con.set("latest_brl_usd", rate.clone()).await?;
//     }
//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     // Create a Redis client without exiting on failure
//     let redis_client = match redis::Client::open("redis://redis:6379/") {
//         Ok(client) => client,
//         Err(err) => {
//             eprintln!("Error connecting to Redis: {}", err);
//             // Proceed without Redis
//             return; // Continue execution, but do not exit the program
//         }
//     };
    
//     // Try to fetch the latest exchange rate from Redis
//     let rate_from_cache = get_exchange_rate_from_redis(&redis_client).await;

//     // Use cached rate if available
//     if let Some(rate) = rate_from_cache {
//         println!("Cached Rate - Date: {}, BRL/USD Rate: {}", rate.data, rate.valor);
//     } else {
//         println!("No cached rate available. Fetching from BCB API...");
//     }

//     // Fetch the latest exchange rate from the BCB API
//     match fetch_brl_usd_exchange_rate().await {
//         Ok(rate) => {
//             println!("Fetched Rate - Date: {}, BRL/USD Rate: {}", rate.data, rate.valor);
//             // Attempt to store the fetched rate in Redis, ignoring errors
//             let _ = set_exchange_rate_to_redis(&redis_client, &rate).await;
//         },
//         Err(err) => eprintln!("Error fetching exchange rate: {}", err),
//     }
// }
