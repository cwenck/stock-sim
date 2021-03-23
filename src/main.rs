use std::collections::HashMap;

use io::read_lines;
use number::Percent;
use pricing::{
    AlternatingPricingStrategy, Period, PriceChange, PriceHistoryDescriptor, PriceHistoryVariants,
    PricingStrategy, SamplingPricingStrategy,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use stats::{
    calculate_statistic, AveragePriceChange, ComputedStatistic, MedianPriceChange,
    StandardDeviationPriceChange, StandardDeviationPriceChangeContext,
};

mod io;
mod number;
mod pricing;
mod stats;

fn load_daily_price_changes() -> Vec<PriceChange> {
    let lines = read_lines("resources/daily-changes.csv").expect("Failed to read file");
    lines
        .map(|line| line.replace(|c: char| !c.is_ascii(), ""))
        .map(|line| {
            let parsed_value = line.parse::<f64>();
            (line, parsed_value)
        })
        .filter_map(|(line, result)| match result {
            Ok(num) => Some(num),
            Err(error) => {
                println!("Parse error [{}] : '{}'", error, line);
                None
            }
        })
        .map(Percent::from_decimal)
        .map(PriceChange::from)
        .collect()
}

const SIMULATIONS: u64 = 10000;

fn main() {
    println!("Running with {} simulations", SIMULATIONS);
    let price_change_options: Vec<PriceChange> = load_daily_price_changes();
    // let price_change_options = vec![
    //     Percent::from_percent(-2.0).into(),
    //     Percent::from_percent(2.05).into(),
    // ];

    let pricing_strategy = SamplingPricingStrategy::new(&price_change_options);
    // let pricing_strategy = AlternatingPricingStrategy::new(&price_change_options);

    let period = Period::Years(5);
    let price_history_variants: Vec<_> = (0..SIMULATIONS)
        .into_par_iter()
        .map(|_| pricing_strategy.calculate_price_history(0..period.as_days()))
        // .inspect(|price_history| println!("{:+.3}", price_history))
        .map(|price_history| PriceHistoryVariants::new(price_history, period))
        .collect();

    let descriptors = Vec::from(price_history_variants[0].descriptors());

    let averages: HashMap<_, _> = calculate_statistic(&price_history_variants, None)
        .map(|value: ComputedStatistic<AveragePriceChange>| {
            (value.descriptor(), value.statistic().average())
        })
        .collect();

    let stdev_context: HashMap<_, _> = averages
        .iter()
        .map(|(&key, &val)| (key, StandardDeviationPriceChangeContext::new(val)))
        .collect();

    let stdevs: HashMap<_, _> = calculate_statistic(&price_history_variants, Some(&stdev_context))
        .map(|value: ComputedStatistic<StandardDeviationPriceChange>| {
            (value.descriptor(), value.statistic().stdev())
        })
        .collect();

    let medians: HashMap<_, _> = calculate_statistic(&price_history_variants, None)
        .map(|value: ComputedStatistic<MedianPriceChange>| {
            (value.descriptor(), value.statistic().median())
        })
        .collect();

    // median.for_each(|item: ComputedStatistic<MedianPriceChange>| {
    //     let median_price_change = item.statistic().median();
    //     let descriptor = item.descriptor();
    //     println!(
    //         "Leverage: {:.1}, Median: {:+.4}",
    //         descriptor.leverage().amount(),
    //         median_price_change.annualized(period)
    //     )
    // });

    let mut stats: Vec<StatGroup> = descriptors
        .iter()
        .map(|descriptor| StatGroup {
            descriptor: *descriptor,
            average: averages[descriptor],
            stdev: stdevs[descriptor],
            median: medians[descriptor],
        })
        .collect();
    stats.sort();

    stats.iter().for_each(|stat_group| {
        println!(
            "Years: {:.1} | Leverage: {: <4.1} | Median: {: >+20.4} | Avg: {: >+20.4} | Stdev: {: >20.4} | Sharpe Ratio: {:.4}",
            stat_group.descriptor.period().as_years(),
            stat_group.descriptor.leverage().amount(),
            stat_group.median.annualized_return(period),
            stat_group.average.annualized_return(period),
            stat_group.stdev.annualized_stdev(period),
            stat_group.average.percent_change().as_decimal() / stat_group.stdev.percent_change().as_decimal()
        )
    });

    // average.iter().for_each(|(descriptor, value)| {
    //     println!(
    //         "Leverage: {:.1}, Avg: {:+.4}",
    //         descriptor.leverage().amount(),
    //         // descriptor.period().as_years().unwrap(),
    //         // avg_price_change,
    //         value.annualized(period)
    //     )
    // });

    // stdev.iter().for_each(|(descriptor, value)| {
    //     println!(
    //         "Leverage: {:.1}, Std-dev: {:+.4}",
    //         descriptor.leverage().amount(),
    //         // descriptor.period().as_years().unwrap(),
    //         // avg_price_change,
    //         value.annualized(period)
    //     )
    // });
}

#[derive(Debug, Clone)]
struct StatGroup {
    descriptor: PriceHistoryDescriptor,
    average: PriceChange,
    stdev: PriceChange,
    median: PriceChange,
}

impl PartialEq for StatGroup {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor.eq(&other.descriptor)
    }
}

impl Eq for StatGroup {}

impl PartialOrd for StatGroup {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StatGroup {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.descriptor.cmp(&other.descriptor)
    }
}
