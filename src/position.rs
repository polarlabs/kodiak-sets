#[cfg(test)]
#[path = "tests/position/tests.rs"]
mod tests;

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

use num_integer::gcd;

use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::AddAssign;

const DENOMINATOR_MIN: u64 = 1;

#[cfg(not(feature = "serde-derive"))]
#[derive(Copy, Clone)]
pub struct Position {
    pub(crate) numerator: u64,
    pub(crate) denominator: u64,
}

#[cfg(feature = "serde-derive")]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Position {
    pub(crate) numerator: u64,
    pub(crate) denominator: u64,
}

pub trait Min {
    const MIN: Position;
}

impl Min for Position {
    const MIN: Position = Position {
        numerator: u64::MIN,
        denominator: 1,
    };
}

pub trait Max {
    const MAX: Position;
}

impl Max for Position {
    const MAX: Position = Position {
        numerator: u64::MAX,
        denominator: 1,
    };
}

impl Position {
    // Creates a valid position, i.e. denominator >= 1
    // If denominator is set to 0, the Position returned will have denominator set to 1.
    pub fn new(numerator: u64, denominator: u64) -> Self {
        let denominator = if denominator < DENOMINATOR_MIN {
            DENOMINATOR_MIN
        } else {
            denominator
        };

        Self { numerator, denominator }
    }

    // Not a valid position, used for incrementing a position only.
    pub(crate) fn n1d0() -> Self {
        Self {
            numerator: 1,
            denominator: 0,
        }
    }

    pub(crate) fn mid(first: Self, second: Self) -> Self {
        Self {
            numerator: first.numerator + second.numerator,
            denominator: first.denominator + second.denominator,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            numerator: self.numerator + other.numerator,
            denominator: self.denominator + other.denominator,
        };
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        if self.numerator == other.numerator && self.denominator == other.denominator {
            true
        } else {
            let self_divisor = gcd(self.numerator, self.denominator);
            let other_divisor = gcd(other.numerator, other.denominator);

            // If greatest common divisor (gcd) is not 1 we have to reduce the
            // Position first by dividing numerator and denominator by the gcd.
            match (self_divisor, other_divisor) {
                (1, 1) => self.numerator == other.numerator && self.denominator == other.denominator,
                (_, 1) => {
                    let f1 = Self::new(self.numerator / self_divisor, self.denominator / self_divisor);
                    f1 == *other
                }
                (1, _) => {
                    let f2 = Self::new(other.numerator / other_divisor, other.denominator / other_divisor);
                    f2 == *self
                }
                (_, _) => {
                    let f1 = Self::new(self.numerator / self_divisor, self.denominator / self_divisor);
                    let f2 = Self::new(other.numerator / other_divisor, other.denominator / other_divisor);
                    f1 == f2
                }
            }
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let f1 = self.numerator as f64 / self.denominator as f64;
        let f2 = other.numerator as f64 / self.denominator as f64;

        f1.partial_cmp(&f2)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} ({})",
            self.numerator,
            self.denominator,
            self.numerator as f64 / self.denominator as f64
        )
    }
}
