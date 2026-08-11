use crate::experiment;
use crate::linear_system::{self, LinearSystem};
use experiment::classical_with_serial_checkup::experienting as cs;
use experiment::gapped_with_serial_checkup::experienting as gs;

use crate::config;
use experiment::*;

// pub fn full(matrix: []u8) void {}

pub fn full(matrix_name: &str) -> Vec<ExperimentResult> {
    let jcwsc: String = String::from("jacobi classical with serial checkups");
    let jgwsc: String = String::from("jacobi gapped with serial checkups");
    let ls = LinearSystem::new(matrix_name);
    let mut id = 0;
    let mut results = Vec::<ExperimentResult>::new();

    for _ in 0..3 {
        let rs = cs(&ls);
        for (performance, config) in rs {
            let metadata = Metadata {
                id,
                algorithm_name: jcwsc.clone(),
                file_name: String::from(matrix_name),
            };
            id += 1;
            results.push(ExperimentResult {
                metadata,
                problem_solution_config: config,
                performance_and_quality: performance,
            });
        }

        let rs = gs(&ls);
        for (performance, config) in rs {
            let metadata = Metadata {
                id,
                algorithm_name: jgwsc.clone(),
                file_name: String::from(matrix_name),
            };
            id += 1;
            results.push(ExperimentResult {
                metadata,
                problem_solution_config: config,
                performance_and_quality: performance,
            });
        }
    }

    results
}
