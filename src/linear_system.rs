//

pub mod checkup;
mod matrix;
pub mod method;
pub mod read;
use super::config;
use matrix::SquareMatrix;
use std::path::Path;

use std::{fs, io};

// s stands for "system" cause "as"s already a reserved word!
#[derive(Debug)]
pub struct LinearSystem {
    sas: SquareMatrix,
    sbs: Vec<f64>,
    slen: usize,
}

impl LinearSystem {
    pub fn new(matrix_name: &str) -> Self {
        let mut as_path = Path::new(config::AS_PATH).join(matrix_name);
        as_path.set_extension("mtx");
        let bs_path = Path::new(config::BS_PATH).join(matrix_name);

        let as_content = fs::read_to_string(&as_path).unwrap_or_else(|err| {
            panic!("Erro ao ler o arquivo A ({:?}): {err}", as_path);
        });

        let sas = read::extract_sas(&as_content);
        let n = sas.len();

        // let opt_bs_content = fs::read_to_string(&bs_path);
        let sbs = read::handle_bs(bs_path, n);
        Self { sas, sbs, slen: n }
    }

    pub unsafe fn initilize_xs(&self) -> Vec<f64> {
        let len = self.slen;
        let mut xs: Vec<f64> = Vec::with_capacity(len);
        unsafe {
            for i in 0..len {
                let bi = *self.sbs.get_unchecked(i);
                let aii = *self.sas.get_unchecked(i, i);
                *xs.get_unchecked_mut(i) = bi / aii;
            }
        }
        xs
    }

    pub unsafe fn jacobi_step(
        &self,
        start_row: usize,
        end_row: usize,
        old_xs: &[f64],
        chunk: &mut [f64],
    ) {
        let len = self.slen;
        debug_assert!(end_row <= len, "end_row > matrix len");
        debug_assert!(old_xs.len() == len, "wrong old_xs len");
        debug_assert!(chunk.len() == end_row - start_row, "wrong chunk len");
        for i in start_row..end_row {
            unsafe {
                let aii = *self.sas.get_unchecked(i, i);
                let mut sum = *self.sbs.get_unchecked(i);
                for j in 0..len {
                    sum -= *self.sas.get_unchecked(i, j) * *old_xs.get_unchecked(j);
                }

                // better than "if j != i {"
                sum += aii * *old_xs.get_unchecked(i);

                // global to local
                *chunk.get_unchecked_mut(i - start_row) = sum / aii;
            }
        }
    }
}
