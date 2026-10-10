// use crate::experiment::
use crate::linear_system::LinearSystem;
use std::mem;
use std::thread;

use crate::linear_system::checkup::classical_serial;

const NUM_THREADS: usize = 32;
const BOUND: u64         = 100_000;

pub enum Convergence {
    Converged, 
    Diverged,
    OutOfBounds,
}

pub fn check(
    linear_system: &LinearSystem,
) -> Convergence {
    let mut converged = false;
    let mut iters: u64 = 0;

    let mut xs = unsafe { linear_system.initilize_xs() };
    let mut buffer: Vec<f64> = vec![0.0; xs.len()];

    let step_size = (xs.len() + NUM_THREADS - 1) / NUM_THREADS;

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
        for &x in &buffer {
            if !x.is_finite() {
               return Convergence::Diverged;
            }
        }

        mem::swap(&mut xs, &mut buffer);
        iters += 1;

        if iters >= BOUND && !converged { return Convergence::OutOfBounds };
    }

    Convergence::Converged
}
