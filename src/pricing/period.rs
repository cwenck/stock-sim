use std::{cmp::Ordering, hash::Hash};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Period {
    Days(u64),
    Years(u32),
}

impl Period {
    pub const MARKET_DAYS_PER_YEAR: u64 = 253;

    pub fn as_days(&self) -> u64 {
        use Period::*;
        match *self {
            Days(val) => val,
            Years(val) => (val as u64) * Period::MARKET_DAYS_PER_YEAR,
        }
    }

    pub fn as_years(&self) -> f64 {
        use Period::*;
        match *self {
            Years(val) => val as f64,
            Days(val) => val as f64 / Period::MARKET_DAYS_PER_YEAR as f64,
        }
    }
}

impl PartialEq for Period {
    fn eq(&self, other: &Self) -> bool {
        self.as_days().eq(&other.as_days())
    }
}

impl Eq for Period {}

impl PartialOrd for Period {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Period {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_days().cmp(&other.as_days())
    }
}

impl Hash for Period {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_days().hash(state);
    }
}
