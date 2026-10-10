use crate::experiment;
use crate::linear_system::{self, LinearSystem};
use experiment::classical_with_serial_checkup::experienting as cs;
use experiment::delayed_with_serial_checkup::experienting as ds;
use experiment::gapped_and_delayed_with_serial_checkup::experienting as gds;
use experiment::gapped_with_serial_checkup::experienting as gs;

use crate::config;
use experiment::*;

// pub fn full(matrix: []u8) void {}

pub fn full(matrix_name: &str) {
    println!("--------------------------------");
    println!("matrix name: {}", matrix_name);
    println!("--------------------------------");
    let jcwsc: String = String::from("jacobi classical with serial checkups");
    let jgwsc: String = String::from("jacobi gapped with serial checkups");
    let jdwsc: String = String::from("jacobi delayed with serial checkups");
    let jgdwsc: String = String::from("jacobi gapped and delayed with serial checkups");
    let ls = LinearSystem::new(matrix_name).unwrap();
    let mut id = 0;
    let mut results = Vec::<ExperimentResult>::new();

    for _ in 0..3 {
        let rs = cs(&ls);
        println!("--------------------------------");
        println!("alg_name: {}", jcwsc);
        println!("--------------------------------");
        for (performance, _) in rs {
            println!("--------------------------------");
            println!("time_spent: {:?}", performance.time_spent);
            println!("iters: {}", performance.num_iterations);
            println!("--------------------------------");
        }

        let rs = gs(&ls);
        println!("--------------------------------");
        println!("alg_name: {}", jgwsc);
        println!("--------------------------------");
        for (performance, _) in rs {
            println!("--------------------------------");
            println!("time_spent: {:?}", performance.time_spent);
            println!("iters: {}", performance.num_iterations);
            println!("--------------------------------");
        }

        let rs = ds(&ls);
        println!("--------------------------------");
        println!("alg_name: {}", jdwsc);
        println!("--------------------------------");
        for (performance, _) in rs {
            println!("--------------------------------");
            println!("time_spent: {:?}", performance.time_spent);
            println!("iters: {}", performance.num_iterations);
            println!("--------------------------------");
        }

        let rs = gds(&ls);
        println!("--------------------------------");
        println!("alg_name: {}", jgdwsc);
        println!("--------------------------------");
        for (performance, _) in rs {
            println!("--------------------------------");
            println!("time_spent: {:?}", performance.time_spent);
            println!("iters: {}", performance.num_iterations);
            println!("--------------------------------");
        }
    }
}
