use crate::calculations::{StandardGravity, ABV}

pub fn abv_from_gravity_diff(
    original_gravity: StandardGravity,
    current_gravity: StandardGravity,
) -> ABV {
     ABV::new((original_gravity - current_gravity) * 131.25)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ext_conversion_values_plato_to_sg() {
        let test_values: Vec<(f32, f32)> = vec![
            ///(OG, FG, ABV)
            (1.055, 1.015, 5.25),
        ];
        for value in test_values {
            let sg = StandardGravity::new(value.0).unwrap();
            let og = StandardGravity::new(value.1).unwrap();
            let abv = abv_from_gravity_diff(sg, og);
            assert!(utils::f32_almost_equal(value.2, abv.value, Some(0.001)));
        }
    }
}
