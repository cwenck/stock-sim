use std::fmt;

use fmt::Debug;

use crate::pricing::{PriceChange, PriceHistory};

pub trait PricingStrategy: Debug + Send + Sync {
    fn calculate_price_change(&self, period: u64, price_history: &PriceHistory) -> PriceChange;

    fn calculate_price_history<R>(&self, range: R) -> PriceHistory
    where
        R: IntoIterator<Item = u64>,
    {
        let mut price_history = PriceHistory::new();

        for period in range {
            let price_change = Self::calculate_price_change(self, period, &price_history);
            price_history.add(price_change);
        }

        price_history
    }
}
