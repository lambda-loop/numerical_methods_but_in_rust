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

// Isso aqui copia e cola o array gigante de structs que o build.rs gerou!
include!(concat!(env!("OUT_DIR"), "/matrizes_hardcoded.rs"));

fn main() {
    // No tempo de execução (runtime), o I/O é 0.
    // O parse é 0.
    // É um array embutido direto no segmento .rodata (Read-Only Data) do binário!

    for matriz in MATRIZES {
        matriz.print();
    }

    println!("Total de matrizes prontas: {}", MATRIZES.len());
}

// fn foo() {}

// fn main() {
//     // let ls = LinearSystem::new("young4c");
//     // let ls = LinearSystem::new("add32");
//     let results = experiment::full::full(matrix_name);

//     let mut csv = String::from(ExpRes::HEADER);
//     for result in results {
//         csv.push('\n');
//         csv.push_str(&result.to_csv());
//     }

//     let out_path = format!("{}{}.csv", OUT_PATH, matrix_name);
//     _ = std::fs::write(&out_path, csv);

//     // println!("{csv:?}");
// }
