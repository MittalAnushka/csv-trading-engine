#[derive(Debug, Clone)]
pub struct Candle {
    pub date: String,
    pub close: f64,
}

#[derive(Debug)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}
