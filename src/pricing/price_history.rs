use core::{
    fmt,
    slice::{Iter, IterMut},
};
use std::{
    fmt::Display,
    iter::FromIterator,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use fmt::Debug;

use crate::number::Percent;

use super::{ExpenseRatio, Leverage, PriceChange};

#[derive(Debug, Clone)]
pub struct PriceHistory {
    price_changes: Vec<PriceChange>,
}

#[allow(dead_code)]
impl PriceHistory {
    pub fn new() -> Self {
        Self {
            price_changes: Vec::new(),
        }
    }

    pub fn add(&mut self, price_change: PriceChange) -> &mut Self {
        self.price_changes.push(price_change);
        self
    }

    pub fn iter(&self) -> Iter<'_, PriceChange> {
        self.price_changes.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, PriceChange> {
        self.price_changes.iter_mut()
    }

    pub fn apply_modifier<M>(mut self, modifier: M) -> Self
    where
        M: PriceHistoryModifier,
    {
        if modifier.modifications_needed() {
            for i in 0..self.price_changes.len() {
                let price_change = self.price_changes[i];
                self.price_changes[i] = modifier.modify_price_change(price_change)
            }
        }
        self
    }

    pub fn leverage_modifier(leverage: Leverage) -> impl PriceHistoryModifier {
        LeverageModifier { leverage }
    }

    pub fn expense_ratio_modifier(expense_ratio: ExpenseRatio) -> impl PriceHistoryModifier {
        ExpenseRatioModifier { expense_ratio }
    }

    pub fn total(&self) -> PriceChange {
        PriceChange::compose_all(&self.price_changes)
    }
}

impl Display for PriceHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, price_change) in self.price_changes.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            Display::fmt(price_change, f)?;
        }
        write!(f, "]")
    }
}

impl<Idx> Index<Idx> for PriceHistory
where
    Idx: SliceIndex<[PriceChange]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.price_changes[index]
    }
}

impl<Idx> IndexMut<Idx> for PriceHistory
where
    Idx: SliceIndex<[PriceChange]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.price_changes[index]
    }
}

impl IntoIterator for PriceHistory {
    type Item = PriceChange;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.price_changes.into_iter()
    }
}

impl FromIterator<PriceChange> for PriceHistory {
    fn from_iter<T: IntoIterator<Item = PriceChange>>(iter: T) -> Self {
        let price_changes: Vec<PriceChange> = iter.into_iter().collect();
        Self { price_changes }
    }
}

impl From<&[PriceChange]> for PriceHistory {
    fn from(price_changes: &[PriceChange]) -> Self {
        Self {
            price_changes: Vec::from(price_changes),
        }
    }
}

pub trait PriceHistoryModifier: Debug {
    fn modify_price_change(&self, price_change: PriceChange) -> PriceChange;
    fn modifications_needed(&self) -> bool;
}

#[derive(Debug, Clone)]
struct LeverageModifier {
    leverage: Leverage,
}

impl PriceHistoryModifier for LeverageModifier {
    fn modify_price_change(&self, price_change: PriceChange) -> PriceChange {
        let updated_decimal_change =
            self.leverage.amount() * price_change.percent_change().as_decimal();
        Percent::from_decimal(updated_decimal_change).into()
    }

    fn modifications_needed(&self) -> bool {
        self.leverage != Leverage::new(1.0)
    }
}
#[derive(Debug, Clone)]
struct ExpenseRatioModifier {
    expense_ratio: ExpenseRatio,
}

impl PriceHistoryModifier for ExpenseRatioModifier {
    fn modify_price_change(&self, price_change: PriceChange) -> PriceChange {
        price_change
            .percent_change()
            .compose(self.expense_ratio.amount())
            .into()
    }

    fn modifications_needed(&self) -> bool {
        f64::abs(self.expense_ratio.amount().as_decimal()) > f64::EPSILON
    }
}
