//
mod config;
mod experiment;
mod linear_system;

use experiment::Metadata;
// use experiment::classical_with_serial_checkup::experienting as exp;
use experiment::ExperimentResult as ExpRes;
use linear_system::LinearSystem;

use config::OUT_PATH;
use linear_system::method::checker::*;
use experiment::jacobi;
pub struct MatrixInfo {
    pub name: &'static str,
    pub len: usize,
    pub is_symmetric: bool,
}

pub fn main() {
    println!("{}", experiment::ExperimentResult::HEADER);
    let ms = all_matrice_names()
        .into_iter()
        .filter(|m_name| !TOO_LONG.contains(&m_name.as_str()));
        // .collect();
    for m in ms {
        jacobi::experiment(&m);
    }
}


use std::fs;
fn create_files(path: &str) {
    fs::create_dir_all(format!("{path}/diverged")).unwrap();
    fs::create_dir_all(format!("{path}/too_long")).unwrap();
    fs::create_dir_all(format!("{path}/bad_formed")).unwrap();
    
}

fn to_trash(matrix_name: &str) {
    let from = format!("data/as/{matrix_name}.mtx");
    let to   = format!("data/as/bad_formed/{matrix_name}.mtx");
    fs::rename(from, to).unwrap();
}

fn move_to(matrix_name: &str, c: Convergence) {
    let from = format!("data/as/{matrix_name}.mtx");
    let to   = match c {
        Convergence::Converged(_) => from.to_string(),
        Convergence::Diverged => format!("data/as/diverged/{matrix_name}.mtx"),
        Convergence::OutOfBounds=> format!("data/as/too_long/{matrix_name}.mtx"),
    };
    fs::rename(from, to).unwrap();
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


fn all_matrice_names() -> Vec<String> {
    let dir_name = config::AS_PATH;
    let mut dir = std::fs::read_dir(dir_name).unwrap();

    let mut ms = Vec::new();
    while let Some(Ok(entry)) = dir.next() {
        let file_type = entry.file_type().unwrap();
        if !file_type.is_file() { continue }
        let m = entry
            .file_name()
            .to_string_lossy()
            .to_string();
        let (m, _) = m.split_once(".").unwrap();

        ms.push(String::from(m));
    }

    ms
}

const TOO_LONG: [&'static str; 3] = [
    "nos7" ,
    "nos2",
    "saylr3",
];

const ALREADY_CHECKED: [&'static str; 6] = [
    "494_bus" ,
    "1138_bus",
    "662_bus" ,
    "bcsstk21",
    "bcsstk09",
    "memplus" ,
];

// jaspiom gap experiemnt
// use experiment::ExperimentResult;
// fn main() {
//     let ms = all_matrice_names();
//     // for m in &ms {
//     //     println!("{m}");
//     // }

//     let lss: Vec<_> = ms
//         .into_iter()
//         .map(|m| {
//             let ls = LinearSystem::new(&m);
//             (m, ls)
//         })
//         .collect();

//     let mut id = 0;
//     println!("{}", experiment::ExperimentResult::HEADER);
//     for (m, ls) in lss {
//         experiment::jaspiom::experienting(&ls)
//             .into_iter()
//             .map(|(performance, config)| {
//                 let metadata = Metadata {
//                     id,
//                     algorithm_name: String::from("usual_jaspiom"), 
//                     file_name: m.clone(),
//                 }; id += 1;

//                 ExperimentResult {
//                     metadata,
//                     performance_and_quality: performance,
//                     problem_solution_config: config,
//                 }.to_csv()
//             }).for_each(|line| {
//                 println!("{line}");
//             });

//         experiment::jaspiomz::experienting(&ls)
//             .into_iter()
//             .map(|(performance, config)| {
//                 let metadata = Metadata {
//                     id,
//                     algorithm_name: String::from("gapped_jaspiom"), 
//                     file_name: m.clone(),
//                 }; id += 1;

//                 ExperimentResult {
//                     metadata,
//                     performance_and_quality: performance,
//                     problem_solution_config: config,
//                 }.to_csv()
//             }).for_each(|line| {
//                 println!("{line}");
//             });
//     }
// }
