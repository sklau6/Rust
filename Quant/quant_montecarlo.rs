extern crate rand;
use rand::Rng;
use std::f64;

// Calculate the payoff
fn payoff(spot: f64, strike: f64) -> f64 {
    let zero: f64 = 0.0;
    return (spot - strike).max(zero);
}

// Simulate the asset price using Geometric Brownian Motion
fn simulate_asset_price(spot: f64, risk_free_rate: f64, volatility: f64, expiry: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let z: f64 = rng.gen::<f64>();
    let drift: f64 = risk_free_rate - 0.5 * f64::powi(volatility, 2);
    return spot * f64::exp(drift * expiry + volatility * f64::sqrt(expiry) * z);
}

fn main() {
    // Set up the parameters
    let spot: f64 = 100.0;
    let strike: f64 = 100.0;
    let risk_free_rate: f64 = 0.05;
    let volatility: f64 = 0.2;
    let expiry: f64 = 1.0; // in years
    let num_simulations: u32 = 1000000;

    // Monte Carlo simulation
    let mut sum_payoff: f64 = 0.0;
    for _ in 0..num_simulations {
        let end_price: f64 = simulate_asset_price(spot, risk_free_rate, volatility, expiry);
        sum_payoff += payoff(end_price, strike);
    }

    // Calculate the average payoff and discount it back to today
    let option_price: f64 = (sum_payoff / num_simulations as f64) * f64::exp(-risk_free_rate * expiry);
    println!("The option price is {}", option_price);
}
