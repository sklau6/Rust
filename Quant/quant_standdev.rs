// Import necessary modules
use std::collections::HashMap;
use std::f64;

// Function to calculate mean of a vector of f64
fn mean(data: &Vec<f64>) -> f64 {
    let sum: f64 = data.iter().sum();
    let count = data.len();

    // Check if count is zero to avoid division by zero
    if count == 0 {
        return 0.0;
    }
    
    sum / count as f64
}

// Function to calculate standard deviation of a vector of f64
fn std_dev(data: &Vec<f64>) -> f64 {
    let avg = mean(data);
    let variance: f64 = data.iter().map(|value| {
        let diff = avg - *value;
        diff * diff
    }).sum::<f64>() / (data.len() as f64);
    
    variance.sqrt()
}

fn main() {
    // Define some stock prices
    let stock_prices = vec![170.23, 165.89, 166.59, 167.43, 167.78, 170.42, 170.89];

    // Calculate and print the mean
    let mean_price = mean(&stock_prices);
    println!("Mean price: {}", mean_price);

    // Calculate and print the standard deviation
    let price_std_dev = std_dev(&stock_prices);
    println!("Price standard deviation: {}", price_std_dev);
}
