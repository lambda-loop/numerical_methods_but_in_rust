// use crate::experiment::
use crate::linear_system::LinearSystem;
use std::mem;
use std::thread;

use crate::linear_system::checkup::classical_serial;

// never call it with num_threads = 1!
pub fn delayed_with_serial_checkup(
    linear_system: &LinearSystem,
    num_threads: usize,
) -> (u64, Vec<f64>) {
    let num_threads = num_threads - 1;
    let mut converged = false;
    let mut iters: u64 = 0;

    let mut xs0 = unsafe { linear_system.initilize_xs() };
    let mut xs1 = vec![0 as f64; linear_system.slen];

    let step_size = (linear_system.slen + num_threads - 1) / num_threads;

    // xs0 -> xs1
    let mut xs2 = thread::scope(|s| {
        for (i, chunk) in xs1.chunks_mut(step_size).enumerate() {
            let start_row = i * step_size;
            let end_row = start_row + chunk.len();

            let xs_ref = &xs0;
            let sys_ref = &linear_system;

            unsafe {
                s.spawn(move || {
                    sys_ref.jacobi_step(start_row, end_row, xs_ref, chunk);
                });
            }
        }

        xs0.clone()
    }); // sync barrier

    while !converged {
        // xs1 -> xs2
        converged = thread::scope(|s| {
            for (i, chunk) in xs2.chunks_mut(step_size).enumerate() {
                let start_row = i * step_size;
                let end_row = start_row + chunk.len();

                let xs_ref = &xs1;
                let sys_ref = &linear_system;

                unsafe {
                    s.spawn(move || {
                        sys_ref.jacobi_step(start_row, end_row, xs_ref, chunk);
                    });
                }
            }

            // on the first iteration, here, the
            // xs0 is in the 0th iteration
            // xs1 is in the 1th iteration
            // xs2 is working on getting into the 2th iteration
            classical_serial(&xs0, &xs1)
        }); // Sync barrier

        mem::swap(&mut xs1, &mut xs2);
        // now xs1 is in the 2th and xs2 is in the 1th
        mem::swap(&mut xs0, &mut xs2);
        // now xs0 is in the 1th and xs2 is in the 0th
        // so now xs with 0th is ready to working on getting into the 3th iteration
        // by using the xs1 in the 2th while the xs0 is in the 1th, whichs ready to compare
        // itself with the xs1.
        iters += 1;
    }

    // note that xs1 is always the best version
    (iters, xs1)
}
