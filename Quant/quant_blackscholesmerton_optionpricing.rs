use std::f64::consts::E;
use statrs::distribution::{Normal, Univariate};

// Function to calculate the cumulative distribution function of a standard normal distribution
fn cumulative_distribution(x: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.cdf(x)
}

// Black-Scholes-Merton option pricing formula
fn black_scholes_merton(stock_price: f64, strike_price: f64, time_to_expiry: f64, risk_free_rate: f64, volatility: f64) -> f64 {
    let d1 = (E.ln(&(stock_price / strike_price)) + (risk_free_rate + volatility.powi(2) / 2.0) * time_to_expiry) / (volatility * (time_to_expiry).sqrt());
    let d2 = d1 - volatility * (time_to_expiry).sqrt();

    let call_option_price = stock_price * cumulative_distribution(d1) - strike_price * E.powf(-risk_free_rate * time_to_expiry) * cumulative_distribution(d2);

    call_option_price
}

fn main() {
    let stock_price = 100.0;  // Current price of the underlying asset
    let strike_price = 100.0;  // Strike price of the option
    let time_to_expiry = 1.0;  // Time to expiry in years
    let risk_free_rate = 0.05;  // Risk-free interest rate
    let volatility = 0.2;  // Volatility of the underlying asset

    // Calculate and print the price of the call option
    let call_option_price = black_scholes_merton(stock_price, strike_price, time_to_expiry, risk_free_rate, volatility);
    println!("The price of the call option is: {:.2}", call_option_price);
}
