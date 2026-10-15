

mod classical;
mod delayed;
mod galayed;
mod gapped;

pub use crate::linear_system::method::jacobi::{
    classical,
    delayed,
    galayed,
    gapped,
};

use crate::experiment;
use crate::linear_system::{LinearSystem};

use crate::config::{NUM_SHOTS};
use experiment::*;
pub fn experiment(matrix_name: &str) {
    let ls = LinearSystem::new(matrix_name).unwrap();
    let mut id = 0;
    // let mut results = Vec::<ExperimentResult>::new();

    for _ in 0..NUM_SHOTS {
        let rs = classical::experienting(&ls);
        for (performance, config) in rs {
            let metadata = Metadata {
                id,
                algorithm_name: String::from("classical"),
                file_name: String::from(matrix_name),
            };
            performance.register_result(&format!("{matrix_name}-classical"), id);
            id += 1;
            let r = ExperimentResult {
                metadata,
                problem_solution_config: config,
                performance_and_quality: performance,
            };

            println!("{}", r.to_csv());
        }

        let rs = gapped::experienting(&ls);
        for (performance, config) in rs {
            let metadata = Metadata {
                id,
                algorithm_name: String::from("gapped"),
                file_name: String::from(matrix_name),
            };

            performance.register_result(&format!("{matrix_name}-gapped"), id);
            id += 1;
            let r = ExperimentResult {
                metadata,
                problem_solution_config: config,
                performance_and_quality: performance,
            };

            println!("{}", r.to_csv());
        }

        let rs = delayed::experienting(&ls);
        for (performance, config) in rs {
            let metadata = Metadata {
                id,
                algorithm_name: String::from("delayed"),
                file_name: String::from(matrix_name),
            };


            performance.register_result(&format!("{matrix_name}-delayed"), id);
            id += 1;
            let r = ExperimentResult {
                metadata,
                problem_solution_config: config,
                performance_and_quality: performance,
            };

            println!("{}", r.to_csv());
        }

        let rs = galayed::experienting(&ls);
        for (performance, config) in rs {
            let metadata = Metadata {
                id,
                algorithm_name: String::from("galayed"),
                file_name: String::from(matrix_name),
            };


            performance.register_result(&format!("{matrix_name}-galayed"), id);
            id += 1;
            let r = ExperimentResult {
                metadata,
                problem_solution_config: config,
                performance_and_quality: performance,
            };

            println!("{}", r.to_csv());
        }
    }
}
