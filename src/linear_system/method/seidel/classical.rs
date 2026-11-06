
use crate::linear_system::LinearSystem;
use crate::linear_system::checkup::classical_serial;
use std::mem;

pub fn method (
    linear_system: &LinearSystem,
) -> (u64, Vec<f64>) {
    let mut converged = false;
    let mut iters = 0_u64;

    let mut xs = unsafe { linear_system.initilize_xs() };
    let xs_len = xs.len();

    let mut buffer = vec![0.0; xs_len];


    while !converged {
        unsafe {
            linear_system.jaspiom_step(
                0,
                xs_len,
                &xs,
                &mut buffer,
            );
        }

        converged = classical_serial(&xs, &buffer);
        mem::swap(&mut xs, &mut buffer);
        iters += 1;
    }

    (iters, xs)
}
