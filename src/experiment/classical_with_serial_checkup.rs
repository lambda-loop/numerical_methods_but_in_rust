// questionable stuff

// use super::*;
use crate::config;
use crate::linear_system::method::classical_with_serial_checkup as method;
use crate::linear_system::LinearSystem;
// use std::time::Duration;

use crate::experiment::PerformanceAndQuality as Performance;
use crate::experiment::ProblemSolutionConfig as Config;

// linear_system: LinearSystem,
// num_threads: usize,
pub fn experienting(linear_system: &LinearSystem) -> Vec<(Performance, Config)> {
    let mut results = Vec::new();

    for num_threads in config::TESTING_THREADS {
        let now = std::time::Instant::now();
        let (iters, final_xs) = method(&linear_system, num_threads);
        let elapsed = now.elapsed();

        // println!("{final_xs:?}");

        let performance = Performance {
            num_iterations: iters,
            time_spent: elapsed,
            final_residual: linear_system.calculate_residual(&final_xs),
            converged: true,
        };

        let config = Config {
            len: linear_system.slen,
            num_threads: num_threads,
            num_threads_c: 1,
            gap: 1,
            delayed: 1,
            sparsed: false,
        };

        results.push((performance, config));
    }

    results
}
