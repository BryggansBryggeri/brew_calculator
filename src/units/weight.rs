use crate::units::dimension;
use std::f32;
use std::ops::{Add, Mul, Sub};

pub trait Weight: dimension::Dimension + Sized {
    fn new(value: f32) -> Result<Self, dimension::Error>;
}

/// Kilogram
#[derive(Debug, Clone, Copy)]
pub struct Kilogram {
    pub value: f32,
    _secret: (),
}

impl dimension::Dimension for Kilogram {
    fn value(self) -> f32 {
        self.value
    }
}

impl Weight for Kilogram {
    fn new(value: f32) -> Result<Kilogram, dimension::Error> {
        if value.is_nan() {
            return Err(dimension::Error::ValueError("NaN value".into()));
        }
        if value.is_sign_negative() {
            return Err(dimension::Error::ValueError(format!(
                "Expected non-negative value, got: {}.",
                value.to_string()
            )));
        }
        Ok(Kilogram { value, _secret: () })
    }
}

impl Mul<f32> for Kilogram {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            value: self.value * rhs,
            _secret: (),
        }
    }
}

impl Add for Kilogram {
    type Output = Self;
    fn add(self, rhs: Kilogram) -> Self::Output {
        Kilogram {
            value: self.value + rhs.value,
            _secret: (),
        }
    }
}

impl Sub for Kilogram {
    type Output = Self;
    fn sub(self, rhs: Kilogram) -> Self::Output {
        Kilogram {
            value: self.value - rhs.value,
            _secret: (),
        }
    }
}

/* Tests
 #[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_ops_test() {
        let weight_1 = Kilogram::new(1.1).unwrap();
        let weight_2 = Kilogram::new(2.3).unwrap();
        assert_eq!((weight_1 + weight_2).value, 3.4);
    }
} */
