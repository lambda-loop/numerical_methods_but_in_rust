// use crate::experiment::
use crate::linear_system::LinearSystem;
use std::mem;
use std::thread;

use crate::linear_system::checkup::classical_serial;

use crate::config;
// use crate::linear_system::LinearSystem;
// use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
// use std::thread; // Para acessar config::EPSI

pub fn jaspiom_with_serial_checkup(
    linear_system: &LinearSystem,
    num_threads: usize,
) -> (u64, Vec<f64>) {
    let system_len = linear_system.slen;
    let mut converged = false;
    let mut iters: u64 = 0;

    let mut xs = unsafe { linear_system.initilize_xs() };
    let mut buffer: Vec<f64> = vec![0.0; system_len];

    let step_size = (system_len + num_threads - 1) / num_threads;

    while !converged {
        thread::scope(|s| {
            for (i, chunk) in xs.chunks_mut(step_size).enumerate() {
                let start_row = i * step_size;
                let end_row = start_row + chunk.len();

                let sys_ref = &linear_system;

                // TODO: optimize
                let mut line_sums = Vec::<f64>::new();
                for i in start_row..end_row {
                    let mut sum = 0 as f64;
                    for j in (0..start_row).chain(end_row..system_len) {
                        unsafe {
                            sum += linear_system.sas.get_unchecked(i, j) * xs.get_unchecked(j)
                        };
                    }
                    line_sums.push(sum);
                }
                // let xs_ref = &xs;
                // let sys_ref = &linear_system;

                unsafe {
                    s.spawn(move || {
                        sys_ref.jaspiom_closure_step(start_row, &line_sums, chunk);
                    });
                }
            }
        }); // Sync Barrier

        // converged = classical_serial(&xs, &buffer);
        // mem::swap(&mut xs, &mut buffer);
        iters += 1;
    }

    (iters, xs)
}

// AI:

pub fn chaotic_jaspiom_in_place(linear_system: &LinearSystem, num_threads: usize) -> Vec<f64> {
    let system_len = linear_system.slen;
    let mut xs = unsafe { linear_system.initilize_xs() };
    let step_size = (system_len + num_threads - 1) / num_threads;

    let is_running = AtomicBool::new(true);
    let converged_count = AtomicUsize::new(0);
    let xs_ptr = xs.as_mut_ptr() as usize;

    thread::scope(|s| {
        for i in 0..num_threads {
            let start_row = i * step_size;
            if start_row >= system_len {
                break;
            }
            let end_row = std::cmp::min(start_row + step_size, system_len);
            let chunk_len = end_row - start_row;

            let sys_ref = linear_system;
            let is_running_ref = &is_running;
            let converged_count_ref = &converged_count;

            s.spawn(move || unsafe {
                sys_ref.chaotic_jaspiom_closure_step(
                    start_row,
                    chunk_len,
                    xs_ptr as *mut f64,
                    is_running_ref,
                    converged_count_ref,
                    num_threads,
                );
            });
        }
    });

    xs
}
