use crate::calculations;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn abv_from_gravity_diff(original_gravity: f32, current_gravity: f32) -> f32 {
    calculations::abv_from_gravity_diff(original_gravity, current_gravity)
}
