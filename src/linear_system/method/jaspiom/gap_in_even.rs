
use crate::linear_system::{
    LinearSystem,
    checkup::classical_serial,
};

use std::{
    mem,
    thread,
}; 

pub fn jaspiomz_with_serial_checkup(
    n_iters      : usize        ,
    linear_system: &LinearSystem,
    num_threads  : usize        ,
) -> (u64, Vec<f64>) {
    debug_assert!(n_iters != 0);
    let system_len = linear_system.slen;
    let mut converged = false;
    let mut iters: u64 = 0;

    let mut xs = unsafe { linear_system.initilize_xs() };
    let mut buffer = vec![0.0; system_len];

    let step_size = (system_len + num_threads - 1) / num_threads;

    while !converged {
        thread::scope(|s| {
            let old_xs = &xs;

            for (i, chunk) in buffer.chunks_mut(step_size).enumerate() {
                let start_row = i * step_size;
                let end_row = start_row + chunk.len();

                s.spawn(move || unsafe {
                    linear_system.jaspiom_steps(
                        n_iters,
                        start_row,
                        end_row,
                        old_xs,
                        chunk,
                    );
                });
            }
        }); // barrier
        converged = classical_serial(&xs, &buffer);
        mem::swap(&mut xs, &mut buffer);

        iters += 1;
    }

    (iters, xs)
}


