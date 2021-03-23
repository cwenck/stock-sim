use rand::Rng;

use crate::pricing::{PriceChange, PriceHistory};

use super::PricingStrategy;

#[derive(Debug, Clone)]
pub struct SamplingPricingStrategy {
    price_change_options: Vec<PriceChange>,
}

#[allow(dead_code)]
impl SamplingPricingStrategy {
    pub fn new(price_change_options: &[PriceChange]) -> Self {
        Self {
            price_change_options: Vec::from(price_change_options),
        }
    }
}

impl PricingStrategy for SamplingPricingStrategy {
    fn calculate_price_change(&self, _period: u64, _price_history: &PriceHistory) -> PriceChange {
        let range = 0..self.price_change_options.len();
        let choice = rand::thread_rng().gen_range(range);
        self.price_change_options[choice]
    }
}
