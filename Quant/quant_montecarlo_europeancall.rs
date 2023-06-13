use rand::distributions::{Distribution, Normal};
use rand::rngs::ThreadRng;
use rand::thread_rng;

// Function to generate a Monte Carlo simulation for a European Call Option
fn monte_carlo_simulation(n: u64, s0: f64, k: f64, t: f64, v: f64, r: f64, rng: &mut ThreadRng) -> f64 {
    // n: Number of simulations
    // s0: Initial stock price
    // k: Strike price
    // t: Time to maturity in years
    // v: Volatility (standard deviation of the stock's returns)
    // r: Risk-free interest rate
    // rng: Random number generator

    let normal_dist = Normal::new(0.0, 1.0);  // Normal distribution N(0, 1)
    let mut sum: f64 = 0.0;  // Sum of the payoff at the end of each simulation

    for _ in 0..n {
        let z: f64 = normal_dist.sample(rng);  // Random variable from the standard normal distribution
        let st = s0 * (r - 0.5 * v.powi(2)) * t + v * (t.sqrt()) * z;  // Stock price at maturity
        let payoff = (st - k).max(0.0);  // Payoff of the call option
        sum += payoff;
    }

    // Calculate the average payoff and discount it back to today
    let option_price = (sum / n as f64) * (-r * t).exp();

    option_price
}

fn main() {
    let mut rng = thread_rng();

    // Option parameters
    let s0 = 100.0;  // Initial stock price
    let k = 100.0;  // Strike price
    let t = 1.0;  // Time to maturity in years
    let v = 0.2;  // Volatility
    let r = 0.05;  // Risk-free interest rate
    let n = 1000000;  // Number of simulations

    let option_price = monte_carlo_simulation(n, s0, k, t, v, r, &mut rng);

    println!("Option price: {:.2}", option_price);
}
