use crate::pricing::{PriceChange, PriceHistory};

use super::PricingStrategy;

#[derive(Debug, Clone)]
pub struct AlternatingPricingStrategy {
    price_change_options: Vec<PriceChange>,
}

#[allow(dead_code)]
impl AlternatingPricingStrategy {
    pub fn new(price_change_options: &[PriceChange]) -> Self {
        Self {
            price_change_options: Vec::from(price_change_options),
        }
    }
}

impl PricingStrategy for AlternatingPricingStrategy {
    fn calculate_price_change(&self, period: u64, _price_history: &PriceHistory) -> PriceChange {
        let choice = period as usize % self.price_change_options.len();
        self.price_change_options[choice]
    }
}
