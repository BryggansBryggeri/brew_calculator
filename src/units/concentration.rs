use crate::units::dimension;
use std::f32;
use std::ops::{Add, Mul, Sub};

pub trait Concentration: dimension::Dimension + dimension::DimensionLess + Sized {
    fn new(value: f32) -> Result<Self, dimension::Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct AlcoholByVolume {
    pub value: f32,
    _secret: (),
}

impl AlcoholByVolume {
    pub fn new(value: f32) -> AlcoholByVolume {
        AlcoholByVolume { value, _secret: () }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpecificGravity {
    pub value: f32,
    _secret: (),
}

impl dimension::Dimension for SpecificGravity {
    fn value(self) -> f32 {
        self.value
    }
}

impl dimension::DimensionLess for SpecificGravity {}

impl Concentration for SpecificGravity {
    fn new(value: f32) -> Result<SpecificGravity, dimension::Error> {
        if value.is_nan() {
            return Err(dimension::Error::ValueError("NaN value".into()));
        }
        if value.is_sign_negative() {
            return Err(dimension::Error::ValueError(format!(
                "Expected non-negative value, got: {}.",
                value.to_string()
            )));
        }
        Ok(SpecificGravity { value, _secret: () })
    }
}

impl<T> Mul<T> for SpecificGravity
where
    T: dimension::Dimension + Mul<f32, Output = T>,
{
    type Output = T;
    fn mul(self, rhs: T) -> Self::Output {
        rhs * self.value
    }
}

impl Mul<f32> for SpecificGravity {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        SpecificGravity {
            value: rhs * self.value,
            _secret: (),
        }
    }
}

impl Add for SpecificGravity {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            _secret: (),
        }
    }
}

impl Sub for SpecificGravity {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
            _secret: (),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Plato {
    pub value: f32,
    _secret: (),
}

impl dimension::Dimension for Plato {
    fn value(self) -> f32 {
        self.value
    }
}

impl dimension::DimensionLess for Plato {}

impl Concentration for Plato {
    fn new(value: f32) -> Result<Plato, dimension::Error> {
        if value.is_nan() {
            return Err(dimension::Error::ValueError("NaN value".into()));
        }
        if value.is_sign_negative() {
            return Err(dimension::Error::ValueError(format!(
                "Expected non-negative value, got: {}.",
                value.to_string()
            )));
        }
        Ok(Plato { value, _secret: () })
    }
}

impl From<SpecificGravity> for Plato {
    /// Unchecked exponentiation, pow() might overflow
    fn from(concentration_sg: SpecificGravity) -> Plato {
        let sg_value = concentration_sg.value;
        let concentration_plato =
            -616.868 + 1111.14 * sg_value - 630.272 * sg_value.powi(2) + 135.997 * sg_value.powi(3);
        Plato {
            value: concentration_plato,
            _secret: (),
        }
    }
}

impl From<Plato> for SpecificGravity {
    fn from(concentration_plato: Plato) -> SpecificGravity {
        let concentration_sg = 1.0
            + (concentration_plato.value / (258.6 - ((concentration_plato.value / 258.2) * 227.1)));
        SpecificGravity {
            value: concentration_sg,
            _secret: (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::volume;
    use crate::units::volume::Volume;
    use crate::utils;
    /// Test some randomly picked values from here:
    /// https://www.brewersfriend.com/plato-to-sg-conversion-chart/
    #[test]
    fn ext_conversion_values_plato_to_sg() {
        let test_values: Vec<(f32, f32)> = vec![
            ///(Plato, SG)
            (0.5, 1.002),
            (2.0, 1.008),
            (6.5, 1.026),
            (15.0, 1.061),
            (39.5, 1.176),
        ];
        for value in test_values {
            let plato = Plato::new(value.0).unwrap();
            let sg: SpecificGravity = plato.into();
            assert!(utils::f32_almost_equal(value.1, sg.value, Some(0.001)));
        }
    }

    /// Test some randomly picked values from here:
    /// https://www.brewersfriend.com/plato-to-sg-conversion-chart/
    #[test]
    fn ext_conversion_values_sg_to_plato() {
        let test_values: Vec<(f32, f32)> = vec![
            ///(Plato, SG)
            (0.5, 1.002),
            (2.0, 1.008),
            (6.5, 1.026),
            (15.0, 1.061),
            (39.5, 1.176),
        ];
        for value in test_values {
            let sg = SpecificGravity::new(value.1).unwrap();
            let plato: Plato = sg.into();
            assert!(utils::f32_almost_equal(value.0, plato.value, Some(0.1)));
        }
    }

    #[test]
    fn add_ops_test() {
        let dens_1 = SpecificGravity::new(1.1).unwrap();
        let dens_2 = SpecificGravity::new(2.3).unwrap();
        assert_eq!((dens_1 + dens_2).value, 3.4);
    }

    #[test]
    fn mul_ops_test() {
        let vol = volume::Litre::new(3.0).unwrap();
        let dens = SpecificGravity::new(1.1).unwrap();
        assert_eq!((dens * vol).value, 3.0 * 1.1);
    }
}
