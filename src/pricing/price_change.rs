use std::fmt::Display;

use crate::number::Percent;

use super::Period;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PriceChange {
    percent_change: Percent,
}

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
        let multiplier = self.percent_change().as_multiplier();
        let annualized_multiplier = f64::powf(multiplier, 1.0 / period.as_years());
        Percent::from_multiplier(annualized_multiplier).into()
    }

    pub fn annualized_stdev(&self, period: Period) -> PriceChange {
        // TODO : Fix this. Seems wrong.
        let decimal = self.percent_change().as_decimal();
        let annualized_decimal = decimal / f64::sqrt(period.as_years());
        Percent::from_decimal(annualized_decimal).into()
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
