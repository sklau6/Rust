use chrono::{DateTime, Utc};
use reqwest;
use serde_json::Value;
use std::collections::HashMap;

async fn fetch_stock_data(ticker: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<HashMap<DateTime<Utc>, f64>, Box<dyn std::error::Error>> {
    // Use the same fetch_stock_data function from the previous example
}

fn ewma(stock_data: &HashMap<DateTime<Utc>, f64>, alpha: f64) -> HashMap<DateTime<Utc>, f64> {
    let mut ewma: HashMap<DateTime<Utc>, f64> = HashMap::new();
    let mut sorted_dates: Vec<&DateTime<Utc>> = stock_data.keys().collect();
    sorted_dates.sort();

    let mut prev_ewma = stock_data[sorted_dates[0]];
    ewma.insert(sorted_dates[0].clone(), prev_ewma);

    for &date in sorted_dates.iter().skip(1) {
        let ewma_value = alpha * stock_data[date] + (1.0 - alpha) * prev_ewma;
        ewma.insert(date.clone(), ewma_value);
        prev_ewma = ewma_value;
    }

    ewma
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ticker = "AAPL";
    let start_date = "2018-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let end_date = "2020-12-31T00:00:00Z".parse::<DateTime<Utc>>().unwrap();

    let stock_data = fetch_stock_data(ticker, start_date, end_date).await?;
    let ewma_data = ewma(&stock_data, 0.1);

    for (date, ewma_value) in &ewma_data {
        println!("{}: {:.2}", date, ewma_value);
    }

    Ok(())
}
