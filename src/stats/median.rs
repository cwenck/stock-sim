use std::usize;

use crate::{
    number::Percent,
    pricing::{PriceChange, PriceHistoryDescriptor},
};

use super::PriceHistoryStatisticValue;

#[derive(Debug, Clone)]
pub struct MedianPriceChange {
    price_changes: Vec<PriceChange>,
    count: u64,
}

impl MedianPriceChange {
    pub fn min(&self) -> PriceChange {
        *self.price_changes.first().unwrap()
    }

    pub fn max(&self) -> PriceChange {
        *self.price_changes.last().unwrap()
    }

    pub fn median(&self) -> PriceChange {
        let half = (self.count / 2) as usize;
        let odd = self.count % 2 == 1;

        if odd {
            self.price_changes[half]
        } else {
            let percent_sum = self.price_changes[half - 1].percent_change()
                + self.price_changes[half].percent_change();
            let avg_decimal = percent_sum.as_decimal() / 2.0;
            Percent::from_decimal(avg_decimal).into()
        }
    }

    pub fn inner_quartile_range(&self) -> PriceChange {
        // TODO : Properly handle this
        let quartile_1 = self
            .percentile(Percent::from_percent(25.0))
            .percent_change();

        let quartile_3 = self
            .percentile(Percent::from_percent(75.0))
            .percent_change();

        (quartile_3 - quartile_1).into()
    }

    pub fn percentile(&self, percentile: Percent) -> PriceChange {
        let index = percentile.as_decimal() * self.count as f64;
        let index = f64::floor(index) as usize;
        self.price_changes[index]
    }
}

impl PriceHistoryStatisticValue for MedianPriceChange {
    type Context = ();

    fn new(
        price_change: PriceChange,
        _descriptor: &PriceHistoryDescriptor,
        _context: Option<&Self::Context>,
    ) -> Self {
        Self {
            price_changes: vec![price_change],
            count: 1,
        }
    }

    fn identity() -> Self {
        Self {
            price_changes: Vec::new(),
            count: 0,
        }
    }

    fn reduce(a: Self, b: Self, _context: Option<&Self::Context>) -> Self {
        let count = a.count + b.count;
        let mut merged_price_changes = Vec::with_capacity(count as usize);

        let mut cursor_a = 0;
        let mut cursor_b = 0;

        while cursor_a < a.price_changes.len() && cursor_b < b.price_changes.len() {
            if a.price_changes[cursor_a] < b.price_changes[cursor_b] {
                merged_price_changes.push(a.price_changes[cursor_a]);
                cursor_a += 1;
            } else {
                merged_price_changes.push(b.price_changes[cursor_b]);
                cursor_b += 1;
            }
        }

        if cursor_a < a.price_changes.len() {
            let remaining = &a.price_changes[cursor_a..];
            merged_price_changes.extend_from_slice(remaining);
        } else if cursor_b < b.price_changes.len() {
            let remaining = &b.price_changes[cursor_b..];
            merged_price_changes.extend_from_slice(remaining);
        }

        Self {
            price_changes: merged_price_changes,
            count,
        }
    }
}
