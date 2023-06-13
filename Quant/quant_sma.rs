// Importing necessary modules
use chrono::{DateTime, Utc};  // Date and Time library
use reqwest;  // HTTP Client library
use serde_json::Value;  // JSON library
use std::collections::VecDeque;  // VecDeque module

// Asynchronous function to fetch stock data
async fn fetch_stock_data(ticker: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<(DateTime<Utc>, f64)>, Box<dyn std::error::Error>> {
    // This function fetches the stock data for a specific ticker in a given date range.
}

// Function to calculate Simple Moving Average (SMA)
fn sma(stock_data: &[(DateTime<Utc>, f64)], window_size: usize) -> Vec<(DateTime<Utc>, f64)> {
    let mut sma: Vec<(DateTime<Utc>, f64)> = Vec::new();
    let mut window: VecDeque<f64> = VecDeque::new();
    let mut sum = 0.0;

    for &(date, price) in stock_data {
        window.push_back(price);
        sum += price;

        if window.len() > window_size {
            if let Some(val) = window.pop_front() {
                sum -= val;
            }
        }

        if window.len() == window_size {
            sma.push((date, sum / window_size as f64));
        }
    }

    sma
}

// Main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the stock ticker, start date, end date and SMA window size
    let ticker = "AAPL";
    let start_date = "2018-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let end_date = "2020-12-31T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let window_size = 20;  // Typical setting for short term trends

    // Fetch the stock data
    let stock_data = fetch_stock_data(ticker, start_date, end_date).await?;

    // Calculate the SMA data
    let sma_data = sma(&stock_data, window_size);

    // Print each date and its corresponding SMA value
    for (date, sma_value) in &sma_data {
        println!("{}: {:.2}", date, sma_value);
    }

    // Return OK
    Ok(())
}
