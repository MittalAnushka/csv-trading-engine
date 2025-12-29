mod data;
mod strategy;
mod backtest;
mod model;

use data::load_csv;
use backtest::run_backtest;

fn main() {
    let prices = load_csv("prices.csv");

    if prices.len() < 6 {
        println!("Not enough data");
        return;
    }

    run_backtest(&prices);
}
