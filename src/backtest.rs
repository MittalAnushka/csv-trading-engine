use crate::model::{Candle, Signal};
use crate::strategy::generate_signal;

pub fn run_backtest(data: &[Candle]) {
    let short = 3;
    let long = 5;

    let mut position = false;
    let mut entry_price = 0.0;
    let mut profit = 0.0;
    let mut trades = 0;

    for i in long..data.len() {
        let short_prices: Vec<f64> =
            data[i - short..i].iter().map(|c| c.close).collect();
        let long_prices: Vec<f64> =
            data[i - long..i].iter().map(|c| c.close).collect();

        let signal = generate_signal(&short_prices, &long_prices);
        let price = data[i].close;
        let date = &data[i].date;

        match signal {
            Signal::Buy if !position => {
                position = true;
                entry_price = price;
                trades += 1;
                println!("{date} | BUY  @ {price}");
            }
            Signal::Sell if position => {
                position = false;
                profit += price - entry_price;
                trades += 1;
                println!("{date} | SELL @ {price}");
            }
            _ => {}
        }
    }

    println!("\n📊 Summary");
    println!("Total Trades: {}", trades);
    println!("Final Profit: {:.2}", profit);
}
