// use crate::experiment::
use crate::linear_system::LinearSystem;
use std::mem;
use std::thread;

use crate::linear_system::checkup::classical_serial;

pub fn method(
    linear_system: &LinearSystem,
    num_threads: usize,
    gap: u8,
) -> (u64, Vec<f64>) {
    let mut converged = false;
    let mut iters: u64 = 0;

    let mut xs = unsafe { linear_system.initilize_xs() };
    let mut buffer: Vec<f64> = vec![0.0; xs.len()];

    let step_size = (xs.len() + num_threads - 1) / num_threads;

    while !converged {
        for _ in 0..gap {
            // PING xs -> buffer
            thread::scope(|s| {
                for (i, chunk) in buffer.chunks_mut(step_size).enumerate() {
                    let start_row = i * step_size;
                    let end_row = start_row + chunk.len();

                    let xs_ref = &xs;
                    let sys_ref = &linear_system;

                    unsafe {
                        s.spawn(move || {
                            sys_ref.jacobi_step(start_row, end_row, xs_ref, chunk);
                        });
                    }
                }
            }); // Sync barrier

            mem::swap(&mut xs, &mut buffer);
        }

        converged = classical_serial(&buffer, &xs);
        iters += gap as u64;
    }

    (iters, xs)
}
