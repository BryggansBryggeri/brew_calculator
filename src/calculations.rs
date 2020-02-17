//! Public API
//!
//! This is the common entry point for both rust and wasm library.

use crate::constants::*;
use wasm_bindgen::prelude::*;

/// Alcohol (ABV) from gravity difference.
///
/// Estimates the alcoholic concentration of the beer,
/// $$
///     C_{ABV} = \kappa (\rho_{OG} - \rho_{CG}),
/// $$
///
/// - $C_{ABV}\ [-]$: Alcohol ratio by volume,
/// - $\rho_{OG}\ [-]$: *Original gravity (OG)*
///
/// - $\rho_{CG}\ [-]$: Current gravity,
/// defined in [SpecificGravity](struct.NotImplentedYet).
///
/// - $\kappa\ [-] = 131.25$: Unit conversion constant.
///
/// Note: if the fermenation is complete, the current gravity is often referred to as the *final
/// gravity (FG)*
///
/// ```
/// # use brew_calculator::calculations::abv_from_gravity_diff;
/// # use assert_approx_eq::assert_approx_eq;
/// let (og, fg) = (1.055, 1.015);
/// let est_abv = abv_from_gravity_diff(og, fg);
/// assert_approx_eq!(est_abv, 5.25, 0.001);
/// ```
#[wasm_bindgen]
pub fn abv_from_gravity_diff(og: f32, current_gravity: f32) -> f32 {
    let gravity_diff = og - current_gravity;
    gravity_diff * GRAVITY_TO_ALCOHOL_COEFF
}

/// Strike water volume from grain weight
#[wasm_bindgen]
pub fn strike_water_volume(grain_weight: f32, mash_thickness: f32) -> f32 {
    grain_weight * mash_thickness
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_abv_from_gravity_diff() {
        let test_values: Vec<(f32, f32, f32)> = vec![
            ///(OG, FG, AlcoholByVolume)
            (1.055, 1.015, 5.25),
        ];
        for value in test_values {
            let sg = value.0;
            let og = value.1;
            let abv = abv_from_gravity_diff(sg, og);
            println!("{}", abv);
            assert_approx_eq!(value.2, abv, 0.001);
        }
    }
}
