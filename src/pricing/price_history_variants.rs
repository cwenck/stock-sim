use core::panic;

use once_cell::sync::Lazy;

use crate::number::Percent;

use super::{ExpenseRatio, Leverage, Period, PriceChange, PriceHistory};

static LEVERAGE_AMOUNTS: Lazy<Vec<Leverage>> = Lazy::new(|| {
    let mut results = Vec::new();
    let mut leverage = 0.0f64;
    while leverage < 10.0 {
        match leverage {
            x if x >= 0.0 && x < 4.0 => {
                leverage += 0.1;
            }
            x if x >= 4.0 && x < 5.0 => {
                leverage += 0.5;
            }
            x if x >= 5.0 && x < 10.0 => {
                leverage += 1.0;
            }
            _ => panic!("Unsupported leverage value: {}", leverage),
        };
        results.push(Leverage::new(leverage));
    }

    results
});

static PERIODS: Lazy<Vec<Period>> = Lazy::new(|| {
    vec![5, 10, 15, 20, 25, 30]
        .into_iter()
        .map(|count| Period::Years(count))
        .collect()
});

pub fn leverage_amounts() -> &'static [Leverage] {
    &LEVERAGE_AMOUNTS
}

pub fn periods() -> &'static [Period] {
    &PERIODS
}

#[derive(Debug, Clone)]
pub struct PriceHistoryVariants {
    total_price_changes: Vec<PriceChange>,
    descriptors: Vec<PriceHistoryDescriptor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PriceHistoryDescriptor {
    leverage: Leverage,
    period: Period,
}

impl PriceHistoryDescriptor {
    pub fn leverage(&self) -> Leverage {
        self.leverage
    }

    pub fn period(&self) -> Period {
        self.period
    }
}

#[allow(dead_code)]
impl PriceHistoryVariants {
    pub fn new(price_history: PriceHistory, period: Period) -> PriceHistoryVariants {
        let total_price_changes = leverage_amounts()
            .iter()
            .copied()
            .map(|leverage| {
                let expense_ratio = expense_ratio_for_leverage(leverage);
                let price_history_variant = price_history
                    .clone()
                    .apply_modifier(PriceHistory::leverage_modifier(leverage))
                    .apply_modifier(PriceHistory::expense_ratio_modifier(expense_ratio));

                // println!(
                //     "Leverage: {:.2}, Expense Ratio: {:.10}, {:+.5}",
                //     leverage.amount(),
                //     expense_ratio.amount(),
                //     price_history_variant
                // );
                price_history_variant
            })
            .map(|price_history_variant| price_history_variant.total())
            .collect();

        let descriptors = leverage_amounts()
            .iter()
            .copied()
            .map(|leverage| PriceHistoryDescriptor { leverage, period })
            .collect();

        PriceHistoryVariants {
            total_price_changes,
            descriptors,
        }
    }

    pub fn descriptors(&self) -> &[PriceHistoryDescriptor] {
        &self.descriptors
    }

    pub fn total_price_changes(&self) -> &[PriceChange] {
        &self.total_price_changes
    }
}

fn expense_ratio_for_leverage(leverage: Leverage) -> ExpenseRatio {
    match leverage.amount() {
        amt if amt > 1.0 => ExpenseRatio::new(Percent::from_percent(0.93), Period::Years(1)),
        _ => ExpenseRatio::zero(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expense_ratio() {
        let no_ratio = ExpenseRatio::zero();
        let high_ratio = ExpenseRatio::new(Percent::from_percent(0.93), Period::Years(1));

        assert_eq!(expense_ratio_for_leverage(Leverage::new(0.0)), no_ratio);
        assert_eq!(expense_ratio_for_leverage(Leverage::new(0.5)), no_ratio);
        assert_eq!(expense_ratio_for_leverage(Leverage::new(1.0)), no_ratio);
        assert_eq!(expense_ratio_for_leverage(Leverage::new(1.5)), high_ratio);
        assert_eq!(expense_ratio_for_leverage(Leverage::new(2.0)), high_ratio);
        assert_eq!(expense_ratio_for_leverage(Leverage::new(3.0)), high_ratio);
        assert_eq!(expense_ratio_for_leverage(Leverage::new(5.0)), high_ratio);
    }

    #[test]
    fn test_leverage_variants() {
        let expected = vec![
            Leverage::new(0.1),
            Leverage::new(0.2),
            Leverage::new(0.3),
            Leverage::new(0.4),
            Leverage::new(0.5),
            Leverage::new(0.6),
            Leverage::new(0.7),
            Leverage::new(0.8),
            Leverage::new(0.9),
            Leverage::new(1.0),
            Leverage::new(1.1),
            Leverage::new(1.2),
            Leverage::new(1.3),
            Leverage::new(1.4),
            Leverage::new(1.5),
            Leverage::new(1.6),
            Leverage::new(1.7),
            Leverage::new(1.8),
            Leverage::new(1.9),
            Leverage::new(2.0),
            Leverage::new(2.1),
            Leverage::new(2.2),
            Leverage::new(2.3),
            Leverage::new(2.4),
            Leverage::new(2.5),
            Leverage::new(2.6),
            Leverage::new(2.7),
            Leverage::new(2.8),
            Leverage::new(2.9),
            Leverage::new(3.0),
            Leverage::new(3.5),
            Leverage::new(4.0),
            Leverage::new(4.5),
            Leverage::new(5.0),
            Leverage::new(6.0),
            Leverage::new(7.0),
            Leverage::new(8.0),
            Leverage::new(9.0),
            Leverage::new(10.0),
        ];
        let actual = Vec::from(leverage_amounts());

        for (i, expected_item) in expected.iter().enumerate() {
            assert_eq!(Some(expected_item), actual.get(i))
        }
    }
}
