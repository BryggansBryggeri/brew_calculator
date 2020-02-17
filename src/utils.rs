//! General brew calculator utilities

/// Linear interpolation of quantity
///
/// $$
///     y(x_t) = y(x_0) + k x_t,
/// $$
/// $$
///     k = \frac{y_T - y_0}{x_T - x_0}
/// $$
pub fn linear_interpolation(
    x_current: f32,
    x_start: f32,
    x_end: f32,
    y_start: f32,
    y_end: f32,
) -> f32 {
    assert!(x_start < x_end);
    let slope = (y_end - y_start) / (x_end - x_start);
    y_start + slope * x_current
}
