
use crate::linear_system::LinearSystem;
use crate::linear_system::checkup::classical_serial;
use std::mem;

pub fn method (
    linear_system: &LinearSystem,
    gap          : u8           ,
) -> (u64, Vec<f64>) {
    let mut converged = false;
    let mut iters = 0_u64;

    let mut xs = unsafe { linear_system.initilize_xs() };
    let xs_len = xs.len();

    let mut buffer = vec![0.0; xs_len];
    let real_gap = gap / 2;

    while !converged {
        for _ in 0..real_gap {
            unsafe {
                linear_system.jaspiom_step(
                    0,
                    xs_len,
                    &xs,
                    &mut buffer,
                );
            }

            unsafe {
                linear_system.jaspiom_step(
                    0,
                    xs_len,
                    &buffer,
                    &mut xs,
                );
            }
        }

        converged = classical_serial(&buffer, &xs);
        iters += gap as u64;
    }

    (iters, xs)
}
