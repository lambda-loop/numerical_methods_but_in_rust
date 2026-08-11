//
mod config;
mod experiment;
mod linear_system;

use experiment::classical_with_serial_checkup::experienting as exp;
use experiment::ExperimentResult as ExpRes;
use linear_system::LinearSystem;

use config::OUT_PATH;
// const matrix_name: &'static str = "dwb512";
const matrix_name: &'static str = "orsirr_2";

fn foo() {}

fn main() {
    // let ls = LinearSystem::new("young4c");
    // let ls = LinearSystem::new("add32");
    let results = experiment::full::full(matrix_name);

    let mut csv = String::from(ExpRes::HEADER);
    for result in results {
        csv.push('\n');
        csv.push_str(&result.to_csv());
    }

    let out_path = format!("{}{}.csv", OUT_PATH, matrix_name);
    _ = std::fs::write(&out_path, csv);

    // println!("{csv:?}");
}
