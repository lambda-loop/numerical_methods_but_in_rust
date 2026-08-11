//

// use crate::experiment::
use crate::linear_system::LinearSystem;
use std::mem;
use std::thread;

use super::checkup::classical_serial;

pub fn classical_with_serial_checkup(
    linear_system: &LinearSystem,
    num_threads: usize,
) -> (u64, Vec<f64>) {
    let mut converged = false;
    let mut iters: u64 = 0;

    let mut xs = unsafe { linear_system.initilize_xs() };
    let mut buffer: Vec<f64> = vec![0.0; xs.len()];

    let step_size = (xs.len() + num_threads - 1) / num_threads;

    while !converged {
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
        }); // Barreira de sincronização (equivalente ao g.await)

        converged = classical_serial(&xs, &buffer);
        mem::swap(&mut xs, &mut buffer);
        iters += 1;
    }

    (iters, xs)
}

pub fn gapped_with_serial_checkup(
    linear_system: &LinearSystem,
    num_threads: usize,
    gap: u8,
) -> (u64, Vec<f64>) {
    debug_assert!(
        gap >= 2 && gap % 2 == 0,
        "Em modo debug: o gap deve ser par e >= 2!"
    );
    let mut converged = false;
    let mut iters: u64 = 0;

    let mut xs = unsafe { linear_system.initilize_xs() };
    let mut buffer: Vec<f64> = vec![0.0; xs.len()];

    let step_size = (xs.len() + num_threads - 1) / num_threads;

    let real_gap = gap / 2;
    while !converged {
        for _ in 0..real_gap {
            // PING xs -> buffer
            thread::scope(|s| {
                for (i, chunk) in buffer.chunks_mut(step_size).enumerate() {
                    let start_row = i * step_size;
                    let end_row = start_row + chunk.len();
                    let xs_ref = &xs;
                    unsafe {
                        s.spawn(move || {
                            linear_system.jacobi_step(start_row, end_row, xs_ref, chunk);
                        });
                    }
                }
            });

            // PONG xs <- buffer
            thread::scope(|s| {
                for (i, chunk) in xs.chunks_mut(step_size).enumerate() {
                    let start_row = i * step_size;
                    let end_row = start_row + chunk.len();
                    let xs_ref = &buffer;
                    unsafe {
                        s.spawn(move || {
                            linear_system.jacobi_step(start_row, end_row, xs_ref, chunk);
                        });
                    }
                }
            });
        }

        converged = classical_serial(&xs, &buffer);
        iters += gap as u64;
    }

    (iters, xs)
}
