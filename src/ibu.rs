//! International bitterness units ($IBU$)
//!
//! - $m$ [kg]: Hop mass,
//! - $\alpha [-]$: Alpha acid (percentage not fraction),
//! - $V$ [l]: Average boil volume (estimated with boil-off),
//! - $t$ [min]: Boil time
//! - $\rho$ [-]: Wort gravity.
use serde::{Deserialize, Serialize};

/// IBU methods
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Method {
    ///See [tinseth_ibu](fn.tinseth_ibu.html)
    Tinseth,
    Rager,
    Garetz,
    Noonan,
}

/// IBU for a single hop addition
///
/// Wrapper for different methods
pub fn ibu(
    method: Method,
    hop_mass: f32,
    alpha_acid: f32,
    volume: f32,
    boil_time: f32,
    wort_gravity: f32,
) -> f32 {
    match method {
        Method::Tinseth => tinseth_ibu(hop_mass, alpha_acid, volume, boil_time, wort_gravity),
        Method::Rager => rager_ibu(hop_mass, alpha_acid, volume, boil_time, wort_gravity),
        Method::Garetz => garetz_ibu(hop_mass, alpha_acid, volume, boil_time, wort_gravity),
        Method::Noonan => noonan_ibu(hop_mass, alpha_acid, volume, boil_time, wort_gravity),
    }
}

/// Tinseth IBU for a single hop addition
///
/// [Reference](https://www.realbeer.com/hops/research.html)
///
/// $$
///     IBU = \frac{m \alpha U(t, \rho)}{V C_{G}(\rho)} \cdot 10^6,
/// $$
///
/// $U(t, \rho)$ and $C_G(\rho)$ is calculated with [`utilisation`](fn.utilisation.html) and
/// [`gravity_correction_factor`](fn.gravity_correction_factor.html) respectively.
pub fn tinseth_ibu(
    hop_mass: f32,
    alpha_acid: f32,
    volume: f32,
    boil_time: f32,
    wort_gravity: f32,
) -> f32 {
    // The original formula has a factor 1000, however here `hop_mass` is measured in kg
    // and the `alpha_acid` in percentage, not a fraction.
    let numerator = 10_000.0 * hop_mass * utilisation(boil_time, wort_gravity) * alpha_acid;
    let denominator = volume * gravity_correction_factor(wort_gravity);
    numerator / denominator
}

/// Rager IBU for a single hop addition
///
/// TODO: Docs should look like `tinset_ibu`
pub fn rager_ibu(
    hop_mass: f32,
    alpha_acid: f32,
    volume: f32,
    boil_time: f32,
    wort_gravity: f32,
) -> f32 {
    todo!();
}

/// Garetz IBU for a single hop addition
///
/// TODO: Docs should look like `tinset_ibu`
pub fn garetz_ibu(
    hop_mass: f32,
    alpha_acid: f32,
    volume: f32,
    boil_time: f32,
    wort_gravity: f32,
) -> f32 {
    todo!();
}

/// Noonan IBU for a single hop addition
///
/// TODO: Docs should look like `tinset_ibu`
pub fn noonan_ibu(
    hop_mass: f32,
    alpha_acid: f32,
    volume: f32,
    boil_time: f32,
    wort_gravity: f32,
) -> f32 {
    todo!();
}

/// Continuous approximation of utilisation factor $U$ [-] for a hop addition.
///
/// https://www.realbeer.com/hops/research.html
///
/// The utilisation is the product of two factors:
/// The 'bigness factor' $C_{big}$ [-] and
/// the 'Boil time factor' $C_{boil}$ [-]:
/// $$
///     U = C_{big}(\rho) C_{boil}(t),
/// $$
/// where
/// $$
///     C_{big} = 1.65 \cdot \left( 1.25 \cdot 10^{-4} \right)^{\rho - 1}
/// $$
/// $$
///     C_{boil} = \frac{1 - e^{-0.04 t}}{4.15}.
/// $$
pub fn utilisation(boil_time: f32, wort_gravity: f32) -> f32 {
    let bigness_factor = 1.65 * 0.000_125_f32.powf(wort_gravity - 1.0);
    let boil_time_factor = (1.0 - (-0.04 * boil_time).exp()) / 4.15;
    bigness_factor * boil_time_factor
}

/// Calculate correction factor for high gravity worts
///
/// https://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
///
/// For wort with gravity $\rho \leq 1.05$, then the correction factor $C_G$
/// is just $1$
/// $$
///     C_G = 1 + \max \left(0, \frac{\rho - 1.05}{ 2 } \right)
/// $$
pub fn gravity_correction_factor(gravity: f32) -> f32 {
    if gravity > 1.05 {
        1.0 + (gravity - 1.05) / 2.0
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    ///https://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
    #[test]
    fn test_utilisation() {
        let calc_util = utilisation(60.0, 1.058);
        assert_approx_eq!(calc_util, 0.211, 0.005);

        let calc_util = utilisation(15.0, 1.058);
        assert_approx_eq!(calc_util, 0.105, 0.005);
    }

    ///https://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
    #[test]
    fn test_gravity_correction_factor() {
        let calc_correction = gravity_correction_factor(1.058);
        assert_approx_eq!(calc_correction, 1.004, 0.005);
    }

    ///https://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
    #[test]
    fn test_tinseth_ibu() {
        let calc_ibu = tinseth_ibu(0.007, 8.5, 22.73, 15.0, 1.058);
        assert_approx_eq!(calc_ibu, 2.74, 0.05);
    }
}
