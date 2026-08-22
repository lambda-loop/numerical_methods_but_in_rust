// use crate::experiment::
use crate::linear_system::LinearSystem;
use std::mem;
use std::thread;

use crate::linear_system::checkup::classical_serial;

// never call it with num_threads <= 2!
// gap should be odd and greater of equals to 3
pub fn gapped_and_delayed_with_checkup_serial(
    linear_system: &LinearSystem,
    num_threads: usize,
    gap: u8,
) -> (u64, Vec<f64>) {
    let num_threads_delayed = num_threads - 1;
    let mut converged = false;
    let mut iters: u64 = 0;

    let mut xs1 = unsafe { linear_system.initilize_xs() };
    // Slow could be improved!
    let mut xs2 = vec![0 as f64; linear_system.slen];
    let mut xs3 = vec![0 as f64; linear_system.slen];

    let step_size = (linear_system.slen + num_threads - 1) / num_threads;
    let step_size_delayed = (linear_system.slen + num_threads_delayed - 1) / num_threads_delayed;

    let real_gap = (gap - 1) / 2;
    while !converged {
        // GAPPED SCOPE

        for _ in 0..real_gap {
            // PING xs1 -> xs2
            thread::scope(|s| {
                for (i, chunk) in xs2.chunks_mut(step_size).enumerate() {
                    let start_row = i * step_size;
                    let end_row = start_row + chunk.len();
                    let xs_ref = &xs1;
                    unsafe {
                        s.spawn(move || {
                            linear_system.jacobi_step(start_row, end_row, xs_ref, chunk);
                        });
                    }
                }
            });

            // PONG xs1 <- xs2
            thread::scope(|s| {
                for (i, chunk) in xs1.chunks_mut(step_size).enumerate() {
                    let start_row = i * step_size;
                    let end_row = start_row + chunk.len();
                    let xs_ref = &xs2;
                    unsafe {
                        s.spawn(move || {
                            linear_system.jacobi_step(start_row, end_row, xs_ref, chunk);
                        });
                    }
                }
            });
        }

        // DELAYED SCOPE
        // xs1 -> xs3
        thread::scope(|s| {
            for (i, chunk) in xs3.chunks_mut(step_size_delayed).enumerate() {
                let start_row = i * step_size_delayed;
                let end_row = start_row + chunk.len();
                let xs_ref = &xs1;
                unsafe {
                    s.spawn(move || {
                        linear_system.jacobi_step(start_row, end_row, xs_ref, chunk);
                    });
                }
            }

            converged = classical_serial(&xs2, &xs1);
        });
        mem::swap(&mut xs1, &mut xs3);
        iters += gap as u64;
    }

    // note that xs1 is always the best version
    (iters, xs1)
}
