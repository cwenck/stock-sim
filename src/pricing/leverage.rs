use core::panic;
use std::hash::Hash;

// Fixed point of 3 decimal places
const FIXED_POINT_MULTIPLIER: f64 = 1000.0;

#[derive(Debug, Clone, Copy)]
pub struct Leverage {
    amount: f64,
    comparable_amount: u32,
}

#[allow(dead_code)]
impl Leverage {
    pub fn new(amount: f64) -> Self {
        if amount < 0.0 {
            panic!("Inavlid leverage amount: {}", amount);
        }

        Leverage {
            amount,
            comparable_amount: (amount * FIXED_POINT_MULTIPLIER).round() as u32,
        }
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}

impl PartialOrd for Leverage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Leverage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.comparable_amount.cmp(&other.comparable_amount)
    }
}

impl PartialEq for Leverage {
    fn eq(&self, other: &Self) -> bool {
        self.comparable_amount.eq(&other.comparable_amount)
    }
}

impl Eq for Leverage {}

impl Hash for Leverage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.comparable_amount.hash(state);
    }
}
