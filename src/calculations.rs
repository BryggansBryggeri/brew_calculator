//! Public API
//!
//! This is the common entry point for both rust and wasm library.

use wasm_bindgen::prelude::*;

/// Alcohol from gravity difference.
///
/// Estimates the alcoholic concentration of the beer,
/// $$
///     C_{ABV} = \kappa (S_O - S_C),
/// $$
///
/// where $C_{ABV}\ [-]$ is the alcohol ratio by volume,
/// $S_G\ [-]$ is the *original gravity (OG)* and $S_C\ [-]$ is the current gravity.
/// $\kappa\ [-] = 131.25$ is a unit conversion constant.
///
/// Note: if the fermenation is complete, the current gravity is often referred to as the *final
/// gravity (FG)* $S_F$.
#[wasm_bindgen]
pub fn abv_from_gravity_diff(original_gravity: f32, current_gravity: f32) -> f32 {
    let gravity_diff = original_gravity - current_gravity;
    gravity_diff * 131.25
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
