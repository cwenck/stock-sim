use std::fmt;

use fmt::Debug;

use crate::{
    number::Percent,
    pricing::{PriceChange, PriceHistoryDescriptor},
    types::Predicate,
};

use super::PriceHistoryStatisticValue;

#[derive(Debug, Clone, Copy)]
pub struct MatchingPriceChangeRatio {
    matching_count: u64,
    count: u64,
}

impl MatchingPriceChangeRatio {
    pub fn success_percent(&self) -> Percent {
        let decimal = self.matching_count as f64 / self.count as f64;
        Percent::from_decimal(decimal)
    }

    pub fn failure_percent(&self) -> Percent {
        let decimal = self.matching_count as f64 / self.count as f64;
        let decimal = 1.0 - decimal;
        Percent::from_decimal(decimal)
    }
}

pub struct MatchingPriceChangeRatioContext {
    predicate: Box<dyn Predicate<PriceChange>>,
}

impl MatchingPriceChangeRatioContext {
    pub fn new<F>(predicate: F) -> Self
    where
        F: 'static + Predicate<PriceChange>,
    {
        Self {
            predicate: Box::new(predicate),
        }
    }
}

impl PriceHistoryStatisticValue for MatchingPriceChangeRatio {
    type Context = MatchingPriceChangeRatioContext;

    fn identity() -> Self {
        Self {
            matching_count: 0,
            count: 0,
        }
    }

    fn new(
        price_change: PriceChange,
        _descriptor: &PriceHistoryDescriptor,
        context: Option<&Self::Context>,
    ) -> Self {
        let predicate = &context.unwrap().predicate;
        let matching_count = match predicate.test(price_change) {
            true => 1,
            false => 0,
        };

        Self {
            matching_count,
            count: 1,
        }
    }

    fn reduce(a: Self, b: Self, _context: Option<&Self::Context>) -> Self {
        Self {
            matching_count: a.matching_count + b.matching_count,
            count: a.count + b.count,
        }
    }
}
