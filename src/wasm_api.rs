use crate::calculations;
use crate::concentration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn abv_from_gravity_diff(original_gravity: f32, current_gravity: f32) -> f32 {
    let original_gravity = concentration::StandardGravity::new(original_gravity);
    let current_gravity = concentration::StandardGravity::new(current_gravity);
    calculations::abv_from_gravity_diff(original_gravity, current_gravity).into()
}
