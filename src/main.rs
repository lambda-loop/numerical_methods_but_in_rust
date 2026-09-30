//
mod config;
mod experiment;
mod linear_system;

use experiment::classical_with_serial_checkup::experienting as exp;
use experiment::ExperimentResult as ExpRes;
use linear_system::LinearSystem;

use classical_with_serial_checkup::classical_with_serial_checkup as jc;
use config::OUT_PATH;
use jaspiom::chaotic_jaspiom_in_place as jm;
use linear_system::method::classical_with_serial_checkup;
use linear_system::method::jaspiom;
// const MATRIX_NAME: &'static str = "dwb512";
const MATRIX_NAME: &'static str = "orsirr_2";
// const MATRIX_NAME: &'static str = "orsirr_1";
// const MATRIX_NAME: &'static str = "add32";

pub struct MatrixInfo {
    pub name: &'static str,
    pub len: usize,
    pub is_symmetric: bool,
}

impl MatrixInfo {
    pub fn print(&self) {
        let sym_text = if self.is_symmetric {
            "symmetric"
        } else {
            "assymetric"
        };
        println!(
            "Matrix[nome: {}, tamanho: {}x{}, tipo: {}]",
            self.name, self.len, self.len, sym_text
        );
    }
}

fn main() {
    // let ls = LinearSystem::new("young4c");
    // let ls = LinearSystem::new("add32");
    // experiment::short::full(matrix_name);
    let ls = LinearSystem::new(MATRIX_NAME);

    let now = std::time::Instant::now();
    let _ = jc(&ls, 12);
    let elapsed = now.elapsed();

    println!("jacobi {elapsed:?}");

    let now = std::time::Instant::now();
    let _ = jm(&ls, 12);
    let elapsed = now.elapsed();

    println!("new_method: {elapsed:?}");
    // let results = experiment::full::full(matrix_name);

    // let mut csv = String::from(ExpRes::HEADER);
    // for result in results {
    //     csv.push('\n');
    //     csv.push_str(&result.to_csv());
    // }

    // let out_path = format!("{}{}.csv", OUT_PATH, matrix_name);
    // _ = std::fs::write(&out_path, csv);

    // println!("{csv:?}");
}
