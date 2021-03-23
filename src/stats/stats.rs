use std::collections::HashMap;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    number::Percent,
    pricing::{PriceChange, PriceHistoryDescriptor, PriceHistoryVariants},
};

pub struct PriceHistoryStatistic<T>
where
    T: PriceHistoryStatisticValue,
{
    depth: usize,
    descriptors: Vec<PriceHistoryDescriptor>,
    values: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct ComputedStatistic<T>
where
    T: Clone,
{
    descriptor: PriceHistoryDescriptor,
    statistic: T,
}

impl<T> ComputedStatistic<T>
where
    T: Clone,
{
    pub fn descriptor(&self) -> PriceHistoryDescriptor {
        self.descriptor
    }

    pub fn statistic(&self) -> &T {
        &self.statistic
    }
}

pub fn calculate_statistic<T, CTX>(
    variants: &[PriceHistoryVariants],
    context: Option<&HashMap<PriceHistoryDescriptor, CTX>>,
) -> impl Iterator<Item = ComputedStatistic<T>>
where
    CTX: Send + Sync,
    T: PriceHistoryStatisticValue<Context = CTX> + Clone,
{
    let mut variant_statistics = variants
        .into_par_iter()
        .cloned()
        .map(|variant| PriceHistoryStatistic::<T>::new(variant, context))
        .reduce_with(|a, b| {
            merge_statistics(a, b, context, |left, right, ctx| {
                T::reduce(left, right, ctx)
            })
        })
        .unwrap();

    (0..variant_statistics.depth).into_iter().map(move |_| {
        let descriptor = variant_statistics.descriptors.remove(0);
        let statistic = variant_statistics.values.remove(0);

        ComputedStatistic {
            descriptor,
            statistic,
        }
    })
}

impl<T> PriceHistoryStatistic<T>
where
    T: PriceHistoryStatisticValue,
{
    fn new(
        variants: PriceHistoryVariants,
        context: Option<&HashMap<PriceHistoryDescriptor, T::Context>>,
    ) -> Self {
        let descriptors = Vec::from(variants.descriptors());
        let values: Vec<T> = variants
            .total_price_changes()
            .iter()
            .copied()
            .enumerate()
            .map(|(i, price_change)| {
                let descriptor = descriptors[i];
                let ctx = context.map(|map| &map[&descriptor]);
                T::new(price_change, ctx)
            })
            .collect();

        debug_assert_eq!(descriptors.len(), values.len());
        let depth = values.len();

        PriceHistoryStatistic {
            depth,
            descriptors,
            values,
        }
    }
}

pub trait PriceHistoryStatisticValue: Send + Sync {
    type Context;

    fn identity() -> Self;
    fn new(price_change: PriceChange, context: Option<&Self::Context>) -> Self;
    fn reduce(a: Self, b: Self, context: Option<&Self::Context>) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct AveragePriceChange {
    price_change_sum: f64,
    count: u64,
}

impl AveragePriceChange {
    pub fn average(&self) -> PriceChange {
        let summed_price_change = self.price_change_sum;
        Percent::from_decimal(summed_price_change / (self.count as f64)).into()
    }
}

impl PriceHistoryStatisticValue for AveragePriceChange {
    type Context = ();

    fn new(price_change: PriceChange, _context: Option<&Self::Context>) -> Self {
        Self {
            price_change_sum: price_change.percent_change().as_decimal(),
            count: 1,
        }
    }

    fn identity() -> Self {
        Self {
            price_change_sum: 0.0,
            count: 0,
        }
    }

    fn reduce(a: Self, b: Self, _context: Option<&Self::Context>) -> Self {
        Self {
            price_change_sum: a.price_change_sum + b.price_change_sum,
            count: a.count + b.count,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StandardDeviationPriceChange {
    variance_sum: f64,
    count: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct StandardDeviationPriceChangeContext {
    average: f64,
}

impl StandardDeviationPriceChangeContext {
    pub fn new(average: PriceChange) -> Self {
        Self {
            average: average.percent_change().as_decimal(),
        }
    }
}

impl StandardDeviationPriceChange {
    pub fn stdev(&self) -> PriceChange {
        let decimal_stdev = f64::sqrt(self.variance_sum / self.count as f64);
        Percent::from_decimal(decimal_stdev).into()
    }
}

impl PriceHistoryStatisticValue for StandardDeviationPriceChange {
    type Context = StandardDeviationPriceChangeContext;

    fn new(price_change: PriceChange, context: Option<&Self::Context>) -> Self {
        let context = context.expect("Expected a context");

        let diff_from_mean = context.average - price_change.percent_change().as_decimal();
        let variance_sum = f64::powi(diff_from_mean, 2);
        Self {
            variance_sum,
            count: 1,
        }
    }

    fn identity() -> Self {
        Self {
            variance_sum: 0.0,
            count: 0,
        }
    }

    fn reduce(a: Self, b: Self, _context: Option<&Self::Context>) -> Self {
        Self {
            variance_sum: a.variance_sum + b.variance_sum,
            count: a.count + b.count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MedianPriceChange {
    price_changes: Vec<PriceChange>,
    count: u64,
}

impl MedianPriceChange {
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
}

impl PriceHistoryStatisticValue for MedianPriceChange {
    type Context = ();

    fn new(price_change: PriceChange, _context: Option<&Self::Context>) -> Self {
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

fn merge_statistics<T, CTX, F>(
    mut stat_a: PriceHistoryStatistic<T>,
    mut stat_b: PriceHistoryStatistic<T>,
    context: Option<&HashMap<PriceHistoryDescriptor, CTX>>,
    reduction_fn: F,
) -> PriceHistoryStatistic<T>
where
    T: PriceHistoryStatisticValue,
    F: Fn(T, T, Option<&CTX>) -> T,
{
    debug_assert_eq!(stat_a.depth, stat_b.depth);
    let depth = stat_a.depth;

    debug_assert_eq!(stat_a.descriptors, stat_b.descriptors);
    let descriptors = stat_a.descriptors;

    let mut values = Vec::with_capacity(depth);
    for i in 0..depth {
        let descriptor = descriptors[i];
        let ctx = context.map(|map| &map[&descriptor]);

        let value_a = stat_a.values.remove(0);
        let value_b = stat_b.values.remove(0);
        values.push(reduction_fn(value_a, value_b, ctx));
    }

    PriceHistoryStatistic {
        depth,
        descriptors,
        values,
    }
}
