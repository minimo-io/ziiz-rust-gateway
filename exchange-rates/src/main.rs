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
