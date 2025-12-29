use crate::model::Signal;

pub fn moving_average(prices: &[f64]) -> f64 {
    prices.iter().sum::<f64>() / prices.len() as f64
}

pub fn generate_signal(
    short_prices: &[f64],
    long_prices: &[f64],
) -> Signal {
    let short_ma = moving_average(short_prices);
    let long_ma = moving_average(long_prices);

    if short_ma > long_ma {
        Signal::Buy
    } else if short_ma < long_ma {
        Signal::Sell
    } else {
        Signal::Hold
    }
}
