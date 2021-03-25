use std::collections::HashMap;

use io::read_lines;
use number::Percent;
use pricing::{
    AlternatingPricingStrategy, Period, PriceChange, PriceHistory, PriceHistoryDescriptor,
    PriceHistoryVariants, PricingStrategy, SamplingPricingStrategy,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use stats::{
    calculate_statistic, AveragePriceChange, ComputedStatistic, MatchingPriceChangeRatio,
    MatchingPriceChangeRatioContext, MedianPriceChange, StandardDeviationPriceChange,
    StandardDeviationPriceChangeContext,
};

mod io;
mod number;
mod pricing;
mod stats;
mod types;

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

const SIMULATIONS: u64 = 10_000;

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

    let averages: HashMap<PriceHistoryDescriptor, AveragePriceChange> =
        calculate_statistic(&price_history_variants, None)
            .map(|value: ComputedStatistic<AveragePriceChange>| {
                (value.descriptor(), value.statistic().clone())
            })
            .collect();

    let stdev_context: HashMap<PriceHistoryDescriptor, StandardDeviationPriceChangeContext> =
        averages
            .iter()
            .map(|(&key, &val)| (key, StandardDeviationPriceChangeContext::new(val.average())))
            .collect();

    let stdevs: HashMap<PriceHistoryDescriptor, StandardDeviationPriceChange> =
        calculate_statistic(&price_history_variants, Some(&stdev_context))
            .map(|value: ComputedStatistic<StandardDeviationPriceChange>| {
                (value.descriptor(), value.statistic().clone())
            })
            .collect();

    let medians: HashMap<PriceHistoryDescriptor, MedianPriceChange> =
        calculate_statistic(&price_history_variants, None)
            .map(|value: ComputedStatistic<MedianPriceChange>| {
                (value.descriptor(), value.statistic().clone())
            })
            .collect();

    let ratio_contexts: HashMap<PriceHistoryDescriptor, MatchingPriceChangeRatioContext> =
        descriptors
            .iter()
            .map(|&descriptor| {
                let context =
                    MatchingPriceChangeRatioContext::new(move |price_change: PriceChange| {
                        price_change.annualized_return(period).percent_change()
                            >= Percent::from_percent(15.0).into()
                    });
                (descriptor, context)
            })
            .collect();

    let _loss_ratios = calculate_statistic::<MatchingPriceChangeRatio, _>(
        &price_history_variants,
        Some(&ratio_contexts),
    )
    .map(|value| (value.descriptor(), value.statistic().clone()))
    .collect::<HashMap<_, _>>();

    let mut stats: Vec<StatGroup> = descriptors
        .iter()
        .map(|descriptor| {
            let percentiles = (0..10)
                .into_iter()
                .map(|i| i as f64 / 20.0)
                .map(|percentile| medians[descriptor].percentile(Percent::from_decimal(percentile)))
                // .map(|price_change| price_change.annualized_return(period))
                .collect::<Vec<_>>();

            StatGroup {
                descriptor: *descriptor,
                average: averages[descriptor].average(),
                annualized_average: averages[descriptor].annualized_average(),
                stdev: stdevs[descriptor].stdev(),
                median: medians[descriptor].median(),
                min: medians[descriptor].min(),
                max: medians[descriptor].max(),
                inner_quartile_range: medians[descriptor].inner_quartile_range(),
                percentiles,
            }
        })
        .collect();
    stats.sort();

    stats.iter().for_each(|stat_group| {
        println!(
            "Years: {:.1} | Leverage: {: <4.1} | Min/Max {:.4} :: {:.4} | IQR: {:.4} |  Percentiles: {:.4}",
            stat_group.descriptor.period().as_years(),
            stat_group.descriptor.leverage().amount(),
            stat_group.min,
            stat_group.max,
            stat_group.inner_quartile_range,
            PriceHistory::from(stat_group.percentiles.as_slice()),
        )
    });
}

#[derive(Debug, Clone)]
struct StatGroup {
    descriptor: PriceHistoryDescriptor,
    average: PriceChange,
    annualized_average: PriceChange,
    stdev: PriceChange,
    median: PriceChange,
    min: PriceChange,
    max: PriceChange,
    inner_quartile_range: PriceChange,
    percentiles: Vec<PriceChange>,
}

impl StatGroup {
    fn sharpe_ratio(&self) -> f64 {
        self.average.percent_change().as_decimal() / self.stdev.percent_change().as_decimal()
    }
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
