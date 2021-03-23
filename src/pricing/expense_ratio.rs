use crate::number::Percent;

use super::Period;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ExpenseRatio {
    amount: Percent,
}

#[allow(dead_code)]
impl ExpenseRatio {
    pub fn new(amount: Percent, period: Period) -> Self {
        let amount_per_period =
            -Percent::from_decimal(amount.as_decimal() / period.as_days() as f64);
        ExpenseRatio {
            amount: amount_per_period,
        }
    }

    pub fn zero() -> Self {
        ExpenseRatio {
            amount: Percent::zero(),
        }
    }

    pub fn amount(&self) -> Percent {
        self.amount
    }

    pub fn multiplier(&self) -> f64 {
        1.0 - self.amount.as_decimal()
    }
}
