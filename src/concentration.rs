use crate::utils;
use std::error as std_error;
use std::f32;

pub trait Concentration: Sized {
    fn new(value: f32) -> Result<Self, Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct SpecificGravity {
    pub value: f32,
    _secret: (),
}

impl Concentration for SpecificGravity {
    fn new(value: f32) -> Result<SpecificGravity, Error> {
        if value.is_nan() {
            return Err(Error::ValueError("NaN value".into()));
        }
        if value.is_sign_negative() {
            return Err(Error::ValueError(format!(
                "Expected non-negative value, got: {}.",
                value.to_string()
            )));
        }
        Ok(SpecificGravity { value, _secret: () })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Plato {
    pub value: f32,
    _secret: (),
}

impl Concentration for Plato {
    fn new(value: f32) -> Result<Plato, Error> {
        if value.is_nan() {
            return Err(Error::ValueError("NaN value".into()));
        }
        if value.is_sign_negative() {
            return Err(Error::ValueError(format!(
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

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ValueError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ValueError(value) => write!(f, "Value must be non-negative: {}", value),
        }
    }
}

impl std_error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ValueError(_) => "Quantity must be non-negative",
        }
    }

    fn cause(&self) -> Option<&dyn std_error::Error> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    /// Test some randomly picked values from here:
    /// https://www.brewersfriend.com/plato-to-sg-conversion-chart/
    #[test]
    fn ext_conversion_values_plato_to_sg() {
        let test_values: Vec<(f32, f32)> = vec![
            //(Plato, SG)
            (0.5, 1.002),
            (2.0, 1.008),
            (6.5, 1.026),
            (15.0, 1.061),
            (39.5, 1.176),
        ];
        for value in test_values {
            let plato = Plato::new(value.0).unwrap();
            let sg: SpecificGravity = plato.into();
            assert_approx_eq!(value.1, sg.value, 0.001);
        }
    }

    /// Test some randomly picked values from here:
    /// https://www.brewersfriend.com/plato-to-sg-conversion-chart/
    /// TODO: Find out why it is off with ~0.5
    #[test]
    fn ext_conversion_values_sg_to_plato() {
        let test_values: Vec<(f32, f32)> = vec![
            //(Plato, SG)
            (0.5, 1.002),
            (2.0, 1.008),
            (6.5, 1.026),
            (15.0, 1.061),
            (39.5, 1.176),
        ];
        for value in test_values {
            let sg = SpecificGravity::new(value.1).unwrap();
            let plato: Plato = sg.into();
            assert_approx_eq!(value.0, plato.value, 0.5);
        }
    }
}
