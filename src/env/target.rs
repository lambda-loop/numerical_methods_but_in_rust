
use super::{AS_PATH, LONG_AS_PATH};
use crate::linear_system::LinearSystem;

use std::env;
use std::fs;

const TABLE_PATH: &'static str = "table";
pub fn setup_ls() -> Option<LinearSystem> {
    let mut args = env::args();
    let arg = env::args()
        .nth(1)
        .expect("missing matrix index");

    let (content, m_name)= get_matrix_content(&arg);
    LinearSystem::from(&content, &m_name)
}

// 41 - "sherman1.mtx"
// LONG - 42 - "494_bus.mtx"
pub fn get_matrix_ext<'a> (line: &'a str) -> MatrixTag<&'a str> {
    let (_, name) = line.split_once("\"").expect("problem with table format");
    let (name, _) = name.split_once("\"").expect("problem with table format");
    println!("{name}");

    if line.starts_with("LONG") {
        Long(name)
    } else {
        Fine(name)
    }
}

pub fn get_matrix_name<'a> (arg: &str, input: &'a str) -> Option<MatrixTag<&'a str>> {
    for line in input.lines() {
        if line.starts_with("LONG") {
            let (clean, m_name) = line
                .strip_prefix("LONG - ").expect("problem with table format")
                .split_once(" - ").expect("problem with table format");
            if arg == clean {
                return m_name.trim_matches("\"")
                    .map(|line| Long(line))
            }
        } else {
            let (clean, m_name) = line
                .split_once(" - ").expect("problem with table format");
            if arg == clean {
                return m_name.trim_matches("\"")
                    .map(|line| Fine(line))
            }
        }
    }

    None
}

fn get_matrix_content(arg: &str) -> (String, String) {
    let table =
        fs::read_to_string(TABLE_PATH)
            .expect("wrong table path");
    let m_name = get_matrix_name(arg, &table).expect("line didnt found in the table");
    let (as_path, m_name) = match m_name {
        Long(m_name) => {
            let m_path = format!("{LONG_AS_PATH}/{m_name}");
            (m_path, m_name)
        },

        Fine(m_name) => {
            let m_path = format!("{AS_PATH}/{m_name}");
            (m_path, m_name)
        },
    };

    let r#as = fs::read_to_string(&as_path).expect("matrix not found. What?");
    (r#as, String::from(m_name))
}

#[derive(Debug)]
pub enum MatrixTag<T> {
    Long(T),
    Fine(T),
}

use MatrixTag::*;
impl<A> MatrixTag<A> {
    fn map<B> (
        self,
        f: impl FnOnce(A) -> B
    ) -> MatrixTag<B> {
        match self {
            Long(m) => Long(f(m)),
            Fine(m) => Fine(f(m)),
        }
    }
}

// 22 - "bcsstm21.mtx"
// 23 - "bcsstm20.mtx"
// 24 - "bcsstm24.mtx"
// 25 - "bcsstm06.mtx"
// 26 - "bcsstm11.mtx"
// 27 - "bcsstm08.mtx"
// 28 - "pde900.mtx"
// 29 - "pde225.mtx"
// 30 - "pde2961.mtx"
// 31 - "fs_541_1.mtx"
// 32 - "jpwh_991.mtx"
// 33 - "orsirr_1.mtx"
// 34 - "orsreg_1.mtx"
// 35 - "orsirr_2.mtx"
// 36 - "watt__1.mtx"
// 37 - "steam3.mtx"
// 38 - "steam2.mtx"
// 39 - "sherman3.mtx"
// 40 - "sherman4.mtx"
// 41 - "sherman1.mtx"
// LONG - 42 - "494_bus.mtx"
// LONG - 43 - "1138_bus.mtx"
// LONG - 44 - "662_bus.mtx"
// LONG - 45 - "bcsstk21.mtx"
// LONG - 46 - "bcsstk09.mtx"
// LONG - 47 - "memplus.mtx"
// LONG - 48 - "nos7.mtx"
// LONG - 49 - "nos1.mtx"
// LONG - 50 - "nos2.mtx"
// LONG - 51 - "nos6.mtx"
// LONG - 52 - "pores_3.mtx"
// LONG - 53 - "saylr1.mtx"
// LONG - 54 - "saylr3.mtx"
// LONG - 55 - "saylr4.mtx"
