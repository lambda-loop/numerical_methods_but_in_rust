//

use super::super::config;
use std::hint::unreachable_unchecked;

pub fn classical_serial(old_xs: &[f64], new_xs: &[f64]) -> bool {
    let mut highest_diff = 0.0f64;
    let mut highest_new_x = 0.0f64;

    if old_xs.len() != new_xs.len() {
        unsafe { unreachable_unchecked() }
    }

    for (&old_x, &new_x) in old_xs.iter().zip(new_xs.iter()) {
        let abs_diff = (old_x - new_x).abs();
        let abs_new_x = new_x.abs();

        if highest_diff < abs_diff {
            highest_diff = abs_diff
        };
        if highest_new_x < abs_new_x {
            highest_new_x = abs_new_x
        };
    }

    highest_new_x == 0.0 || highest_diff / highest_new_x < config::EPSI
}
