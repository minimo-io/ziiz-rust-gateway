use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct ExchangeRate {
    data: String,
    valor: String,
}

/// Retrieves the latest BRL/USD exchange rate from the BCB API.
async fn fetch_brl_usd_exchange_rate() -> Result<ExchangeRate, Error> {
    // Define the URL for the BCB API endpoint that provides the exchange rates
    let url = "https://api.bcb.gov.br/dados/serie/bcdata.sgs.10813/dados?formato=json";

    // Send a GET request to the API and await the response as JSON
    let response = reqwest::get(url).await?.json::<Vec<ExchangeRate>>().await?;

    // Extract the latest exchange rate from the response (the last element in the array)
    let latest_rate = response.last().unwrap().clone();

    Ok(latest_rate)
}

#[tokio::main]
async fn main() {
    // Fetch and print the latest BRL/USD exchange rate from the BCB API
    match fetch_brl_usd_exchange_rate().await {
        Ok(rate) => println!("Date: {}, BRL/USD Rate: {}", rate.data, rate.valor),
        Err(err) => eprintln!("Error fetching exchange rate: {}", err),
    }
}
