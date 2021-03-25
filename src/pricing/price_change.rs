use std::fmt::Display;

use crate::number::Percent;

use super::Period;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PriceChange {
    percent_change: Percent,
}

// TODO : make a type distinction between an annualized price change, a daily price change, and a total price change

#[allow(dead_code)]
impl PriceChange {
    pub fn new(percent_change: Percent) -> PriceChange {
        percent_change.into()
    }

    pub fn zero() -> PriceChange {
        Percent::zero().into()
    }

    pub fn total_loss() -> PriceChange {
        PriceChange {
            percent_change: Percent::from_percent(-100.0),
        }
    }

    pub fn percent_change(&self) -> Percent {
        self.percent_change
    }

    pub fn annualized_return(&self, period: Period) -> PriceChange {
        let total_multiplier = self.percent_change().as_multiplier();
        let annualized_multiplier = f64::powf(total_multiplier, 1.0 / period.as_years());
        Percent::from_multiplier(annualized_multiplier).into()
    }

    /// Computes the total return from an annualized price change.
    pub fn total_return(&self, period: Period) -> PriceChange {
        let annualized_multiplier = self.percent_change().as_multiplier();
        let total_multiplier = f64::powf(annualized_multiplier, period.as_years());
        Percent::from_multiplier(total_multiplier).into()
    }

    pub fn compose(&self, other: PriceChange) -> PriceChange {
        self.percent_change.compose(other.percent_change).into()
    }

    pub fn compose_all(price_changes: &[PriceChange]) -> PriceChange {
        let percent_changes: Vec<_> = price_changes
            .iter()
            .map(|price_change| price_change.percent_change)
            .collect();

        Percent::compose_all(&percent_changes).into()
    }
}

impl Display for PriceChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.percent_change.fmt(f)
    }
}

impl From<Percent> for PriceChange {
    fn from(percent: Percent) -> Self {
        PriceChange {
            percent_change: percent,
        }
    }
}

impl From<PriceChange> for Percent {
    fn from(price_change: PriceChange) -> Self {
        price_change.percent_change
    }
}
