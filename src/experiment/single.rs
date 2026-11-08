use super::{ExperimentResult, Metadata, PerformanceAndQuality, ProblemSolutionConfig};
use crate::config::{TESTING_GAPS, TESTING_THREADS};
use crate::linear_system::{method::{jacobi, jaspiom, seidel}, LinearSystem};
use std::time::Instant;

pub fn run(matrix_name: &str, linear_system: &LinearSystem, num_shots: usize) {
    let mut id = 0_u64;

    for shot in 1..=num_shots {
        for num_threads in TESTING_THREADS {
            run_one(matrix_name, linear_system, &mut id, shot,
                "jacobi_classical", (num_threads, 1, 1),
                || jacobi::classical(linear_system, num_threads));
        }
        for gap in TESTING_GAPS {
            for num_threads in TESTING_THREADS {
                run_one(matrix_name, linear_system, &mut id, shot,
                    "jacobi_gap_even", (num_threads, 1, gap as usize),
                    || jacobi::gapped(linear_system, num_threads, gap));
            }
        }

        run_one(matrix_name, linear_system, &mut id, shot,
            "seidel_classical", (1, 1, 1),
            || seidel::classical(linear_system));
        for gap in TESTING_GAPS {
            run_one(matrix_name, linear_system, &mut id, shot,
                "seidel_gap_even", (1, 1, gap as usize),
                || seidel::gap_even(linear_system, gap));
        }

        for num_threads in TESTING_THREADS {
            run_one(matrix_name, linear_system, &mut id, shot,
                "jaspiom_classical", (num_threads, 1, 1),
                || jaspiom::classical(linear_system, num_threads));
        }
        for gap_in in TESTING_GAPS {
            for num_threads in TESTING_THREADS {
                run_one(matrix_name, linear_system, &mut id, shot,
                    "jaspiom_gap_in_even", (num_threads, gap_in as usize, 1),
                    || jaspiom::gap_in_even(gap_in as usize, linear_system, num_threads));
            }
        }
        for gap_out in TESTING_GAPS {
            for num_threads in TESTING_THREADS {
                run_one(matrix_name, linear_system, &mut id, shot,
                    "jaspiom_gap_out_even", (num_threads, 1, gap_out as usize),
                    || jaspiom::gap_out_even(linear_system, num_threads, gap_out));
            }
        }
        for gap_in in TESTING_GAPS {
            for gap_out in TESTING_GAPS {
                for num_threads in TESTING_THREADS {
                    run_one(matrix_name, linear_system, &mut id, shot,
                        "jaspiom_gap_mixed_even",
                        (num_threads, gap_in as usize, gap_out as usize),
                        || jaspiom::gap_mixed_even(
                            gap_in as usize, linear_system, num_threads, gap_out));
                }
            }
        }
    }
}

fn run_one(
    matrix_name: &str,
    linear_system: &LinearSystem,
    id: &mut u64,
    shot: usize,
    algorithm_name: &str,
    (num_threads, gap_in, gap_out): (usize, usize, usize),
    solve: impl FnOnce() -> (u64, Vec<f64>),
) {
    let now = Instant::now();
    let (num_iterations, final_result) = solve();
    let time_spent = now.elapsed();

    let final_residual = linear_system.calculate_residual(&final_result);
    let performance_and_quality = PerformanceAndQuality {
        num_iterations,
        time_spent,
        final_result,
        final_residual,
        converged: true,
    };
    performance_and_quality.register_result(
        &format!("{matrix_name}-{algorithm_name}"), *id);

    let result = ExperimentResult {
        metadata: Metadata {
            id: *id,
            algorithm_name: String::from(algorithm_name),
            file_name: String::from(matrix_name),
        },
        problem_solution_config: ProblemSolutionConfig {
            len: linear_system.slen,
            num_threads,
            num_threads_c: 1,
            gap: if gap_out == 1 { gap_in } else { gap_out },
            delayed: 1,
            sparsed: false,
        },
        performance_and_quality,
    };
    println!("{},{shot},{gap_in},{gap_out}", result.to_csv());
    *id += 1;
}
