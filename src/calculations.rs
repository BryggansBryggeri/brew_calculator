use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn abv_from_gravity_diff(original_gravity: f32, current_gravity: f32) -> f32 {
    let gravity_diff = original_gravity - current_gravity;
    gravity_diff * 131.25
}

#[wasm_bindgen]
pub fn strike_water_volume(grain_weight: f32, mash_thickness: f32) -> f32 {
    grain_weight * mash_thickness
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
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
            assert!(utils::f32_almost_equal(value.2, abv, Some(0.001)));
        }
    }
}
