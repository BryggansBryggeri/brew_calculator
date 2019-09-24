use crate::units::dimension;
use std::f32;
use std::ops::{Add, Mul};

pub trait Volume: dimension::Dimension + Sized {
    fn new(value: f32) -> Result<Self, dimension::Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct Litre {
    pub value: f32,
    _secret: (),
}

impl dimension::Dimension for Litre {
    fn value(self) -> f32 {
        self.value
    }
}

impl Volume for Litre {
    fn new(value: f32) -> Result<Litre, dimension::Error> {
        if value.is_nan() {
            return Err(dimension::Error::ValueError("NaN value".into()));
        }
        if value.is_sign_negative() {
            return Err(dimension::Error::ValueError(format!(
                "Expected non-negative value, got: {}.",
                value.to_string()
            )));
        }
        Ok(Litre { value, _secret: () })
    }
}

impl Mul<f32> for Litre {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            value: self.value * rhs,
            _secret: (),
        }
    }
}

impl Add for Litre {
    type Output = Self;
    fn add(self, rhs: Litre) -> Self::Output {
        Litre {
            value: self.value + rhs.value,
            _secret: (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_ops_test() {
        let vol_1 = Litre::new(1.1).unwrap();
        let vol_2 = Litre::new(2.3).unwrap();
        assert_eq!((vol_1 + vol_2).value, 3.4);
    }
}
