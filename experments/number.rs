use std::ops::Range;

use actix_web::cookie::Display;
use fpdec::CheckedMul;
use num::{range_inclusive, range_step, Integer};

/// a real valued floating point number
pub struct Real64(f64);

/// a fixed point integer with aprox base10 9 integer, 9 fractional
pub type FixedI32F32 = fixed::types::I32F32;

/// a fixed point integer with aprox base10 12 integer, 6 fractional
pub type FixedI44F20 = fixed::types::I44F20;

/// a fixed point integer with aprox base10 16 integer, 3 fractional
pub type FixedI54F10 = fixed::types::I54F10;

/// a fixed point integer with aprox base10 6 integer, 3 fractional
pub type FixedI22F10 = fixed::types::I22F10;

/// a fixed point integer with aprox base10 1 integer, 3 fractional
pub type FixedI8F8 = fixed::types::I6F10;

// pub fn linspace(start: f64, end: f64, step: u64) -> Vec<f64> {

// }

struct DecimalMilli {
    value: i64,
}

impl std::fmt::Display for DecimalMilli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_f3())
    }
}

impl From<i32> for DecimalMilli {
    fn from(value: i32) -> Self {
        Self::from_integer_unchecked(value as i64)
    }
}

enum DecimalMilliError {
    Overflow,
}

// can't do this because of the associated type
// impl IntoIterator for Range<DecimalMilli> {
//     type Item = DecimalMilli;

//     type IntoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

impl DecimalMilli {
    pub const EPSILON: Self = Self { value: 1 };
    pub const ZERO: Self = Self { value: 0 };
    const ONE_VALUE: i64 = 1000;
    pub const ONE: Self = Self { value: Self::ONE_VALUE };


    /// Create a new DecimalMilli from an integer value
    /// Returns None if the value is too large to fit in a DecimalMilli
    fn from_integer(value: i64) -> Option<Self> {
        let value = value.checked_mul(Self::ONE_VALUE)?;
        Some(Self { value })
    }

    fn from_integer_unchecked(value: i64) -> Self {
        let value = value * Self::ONE_VALUE;
        Self { value }
    }

    pub fn new_from_millis(value: i64) -> Self {
        Self { value }
    }

    pub fn display_f3(&self) -> String {
        let (i, f) = self.value.div_rem(&Self::ONE_VALUE);

        format!("{}.{:03}", i, f.abs())
    }

    pub fn range_step(start: Self, stop: Self, step: Self) -> impl Iterator<Item = Self> {
        let values = range_step(start.value, stop.value, step.value);
        values.map(Self::new_from_millis)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use num::{range_step, range_step_inclusive, Integer, Signed};

    use super::*;

    #[test]
    fn test_fixed() {
        let max = FixedI8F8::MAX;
        let min = FixedI8F8::MIN;
        let step = FixedI8F8::lit("0.001");
        let reprs: HashSet<_> = range_step_inclusive(min, max, step)
            .map(|v| format!("{:.3}", v))
            .collect();

        for v in DecimalMilli::range_step(
            DecimalMilli::from(-32),
            DecimalMilli::from(32),
            DecimalMilli::EPSILON,
        ) {
            let repr = v.display_f3();
            assert!(reprs.contains(&repr), "missing: {}", repr);
        }

        // let expected_num_reprs = (max_value - min_value) / step_value;
        // let got_num_reprs = reprs.len();
        // println!("expected: {}", expected_num_reprs);
        // println!("got: {}", got_num_reprs);
    }
}

pub struct Decimal128Nano(fpdec::Decimal);
pub struct Decimal128Milli(fpdec::Decimal);

impl TryFrom<f64> for Real64 {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err("value is not finite")
        }
    }
}

pub enum Number {
    I64(i64),
    U64(u64),
    // Real(f64),
}
