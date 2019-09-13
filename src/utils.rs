use std::f32;

pub fn f32_almost_equal(a: f32, b: f32, tolerance: Option<f32>) -> bool {
    let tolerance = tolerance.unwrap_or(f32::EPSILON);
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < (tolerance * f32::MIN_POSITIVE)
    } else {
        // Use relative error.
        (diff / f32::min(abs_a + abs_b, f32::MAX)) < tolerance
    }
}
