// Importing necessary modules
use chrono::{DateTime, Utc};  // Date and Time library
use reqwest;  // HTTP Client library
use serde_json::Value;  // JSON library
use std::collections::HashMap;  // Hashmap module

// Asynchronous function to fetch stock data
async fn fetch_stock_data(ticker: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<HashMap<DateTime<Utc>, f64>, Box<dyn std::error::Error>> {
    // This function fetches the stock data for a specific ticker in a given date range.
}

// Function to calculate Exponential Weighted Moving Average (EWMA)
fn ewma(stock_data: &HashMap<DateTime<Utc>, f64>, alpha: f64) -> HashMap<DateTime<Utc>, f64> {
    // Instantiate a new HashMap to hold the ewma values
    let mut ewma: HashMap<DateTime<Utc>, f64> = HashMap::new();

    // Collect the dates from the stock data and sort them in ascending order
    let mut sorted_dates: Vec<&DateTime<Utc>> = stock_data.keys().collect();
    sorted_dates.sort();

    // Initialize the previous EWMA to the stock data at the first date
    let mut prev_ewma = stock_data[sorted_dates[0]];
    ewma.insert(sorted_dates[0].clone(), prev_ewma);

    // Iterate over the dates, skipping the first one
    for &date in sorted_dates.iter().skip(1) {
        // Calculate the EWMA value for the current date
        let ewma_value = alpha * stock_data[date] + (1.0 - alpha) * prev_ewma;
        
        // Store the EWMA value in the HashMap
        ewma.insert(date.clone(), ewma_value);

        // Update the previous EWMA value for the next iteration
        prev_ewma = ewma_value;
    }

    // Return the HashMap of EWMA values
    ewma
}

// Main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the stock ticker, start date and end date
    let ticker = "AAPL";
    let start_date = "2018-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let end_date = "2020-12-31T00:00:00Z".parse::<DateTime<Utc>>().unwrap();

    // Fetch the stock data
    let stock_data = fetch_stock_data(ticker, start_date, end_date).await?;
    
    // Calculate the EWMA data
    let ewma_data = ewma(&stock_data, 0.1);

    // Print each date and its corresponding EWMA value
    for (date, ewma_value) in &ewma_data {
        println!("{}: {:.2}", date, ewma_value);
    }

    // Return OK
    Ok(())
}
