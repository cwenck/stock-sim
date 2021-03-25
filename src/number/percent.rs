use std::{
    fmt::Display,
    ops::{self, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(f64);

#[allow(dead_code)]
impl Percent {
    pub fn zero() -> Percent {
        Percent::from_decimal(0.0)
    }

    pub fn from_percent(percentage: f64) -> Percent {
        Percent(percentage / 100.0)
    }

    pub fn from_decimal(decimal: f64) -> Percent {
        Percent(decimal)
    }

    pub fn from_multiplier(multipler: f64) -> Percent {
        let decimal = match multipler {
            x if x < 0.0 => -1.0,
            _ => multipler - 1.0,
        };

        Percent(decimal)
    }

    pub fn as_multiplier(&self) -> f64 {
        self.0 + 1.0
    }

    pub fn as_decimal(&self) -> f64 {
        self.0
    }

    pub fn as_percent(&self) -> f64 {
        self.0 * 100.0
    }

    pub fn compose(&self, other: Percent) -> Percent {
        let composed_multiplier = self.as_multiplier() * other.as_multiplier();
        Self::from_multiplier(composed_multiplier)
    }

    pub fn compose_all(percents: &[Percent]) -> Percent {
        let composed_multiplier = percents
            .iter()
            .map(Percent::as_multiplier)
            .fold(1.0, f64::mul);
        Self::from_multiplier(composed_multiplier)
    }
}

impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_percent().fmt(f)?;
        write!(f, "%")
    }
}

impl Add<Percent> for Percent {
    type Output = Percent;

    fn add(self, rhs: Percent) -> Self::Output {
        Percent(self.0 + rhs.0)
    }
}

impl AddAssign<Percent> for Percent {
    fn add_assign(&mut self, rhs: Percent) {
        self.0 += rhs.0;
    }
}

impl Mul<Percent> for Percent {
    type Output = Percent;

    fn mul(self, rhs: Percent) -> Self::Output {
        Percent(self.0 * rhs.0)
    }
}

impl MulAssign<Percent> for Percent {
    fn mul_assign(&mut self, rhs: Percent) {
        self.0 *= rhs.0;
    }
}

impl Neg for Percent {
    type Output = Percent;

    fn neg(self) -> Self::Output {
        Percent(-self.0)
    }
}

impl Sub<Percent> for Percent {
    type Output = Percent;

    fn sub(self, rhs: Percent) -> Self::Output {
        Percent(self.0 - rhs.0)
    }
}

impl SubAssign<Percent> for Percent {
    fn sub_assign(&mut self, rhs: Percent) {
        self.0 -= rhs.0;
    }
}
