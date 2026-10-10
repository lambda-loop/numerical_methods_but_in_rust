//
mod config;
mod experiment;
mod linear_system;

use experiment::Metadata;
use experiment::classical_with_serial_checkup::experienting as exp;
use experiment::ExperimentResult as ExpRes;
use linear_system::LinearSystem;

use classical_with_serial_checkup::classical_with_serial_checkup as jc;
use config::OUT_PATH;
// use jaspiom::chaotic_jaspiom_in_place as jm;
use linear_system::method::classical_with_serial_checkup;
use linear_system::method::jaspiom;
// const MATRIX_NAME: &'static str = "dwb512";
const MATRIX_NAME: &'static str = "orsirr_2";
// const MATRIX_NAME: &'static str = "orsirr_1";
// const MATRIX_NAME: &'static str = "add32";

use linear_system::method::checker::*;
pub struct MatrixInfo {
    pub name: &'static str,
    pub len: usize,
    pub is_symmetric: bool,
}

pub fn main() {
    let ms = all_matrice_names();

    let lss: Vec<_> = ms
        .into_iter()
        .map(|m| {
            let ls = LinearSystem::new(&m);
            (m, ls)
        })
        // .filter(|(_, opt)| opt.is_some())
        // .map(|(m, opt)| (m, opt.unwrap()))
        .collect();

    create_files("data/as");
    for (m, opt_ls) in lss {
        if let Some(ls) = opt_ls {
            let c = check(&ls);
            move_to(&m, c);
        } else { to_trash(&m); }
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
        Convergence::Converged => from.to_string(),
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
    let dir_name = "data/as";
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
