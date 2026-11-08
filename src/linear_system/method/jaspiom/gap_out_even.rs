use crate::linear_system::{checkup::classical_serial, LinearSystem};
use std::thread;

// AI GENERATED
/// Checks convergence every `gap` global JASPIoM iterations.
/// `gap` is even and at least two; `num_threads` is positive.
pub fn method(
    linear_system: &LinearSystem,
    num_threads: usize,
    gap: u8,
) -> (u64, Vec<f64>) {
    let system_len = linear_system.slen;
    let mut converged = false;
    let mut iters = 0_u64;

    let mut xs = unsafe { linear_system.initilize_xs() };
    let mut buffer = vec![0.0; system_len];

    let step_size = (system_len + num_threads - 1) / num_threads;
    let real_gap = gap / 2;

    while !converged {
        for _ in 0..real_gap {
            // PING: xs -> buffer
            thread::scope(|s| {
                let old_xs = &xs;

                for (i, chunk) in buffer.chunks_mut(step_size).enumerate() {
                    let start_row = i * step_size;
                    let end_row = start_row + chunk.len();

                    s.spawn(move || unsafe {
                        linear_system.jaspiom_step(start_row, end_row, old_xs, chunk);
                    });
                }
            });

            // PONG: buffer -> xs
            thread::scope(|s| {
                let old_xs = &buffer;

                for (i, chunk) in xs.chunks_mut(step_size).enumerate() {
                    let start_row = i * step_size;
                    let end_row = start_row + chunk.len();

                    s.spawn(move || unsafe {
                        linear_system.jaspiom_step(start_row, end_row, old_xs, chunk);
                    });
                }
            });
        }

        converged = classical_serial(&buffer, &xs);
        iters += gap as u64;
    }

    (iters, xs)
}
