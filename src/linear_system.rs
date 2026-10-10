//

pub mod adjacency_list;
pub mod checkup;
mod matrix;
pub mod method;
pub mod read;
use super::config;
use matrix::SquareMatrix;
use std::path::Path;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{fs, io};

// s stands for "system" cause "as"s already a reserved word!
#[derive(Debug)]
pub struct LinearSystem {
    sas: SquareMatrix,
    sbs: Vec<f64>,
    pub slen: usize,
}

impl LinearSystem {
    pub fn new(matrix_name: &str) -> Option<Self> {
        let mut as_path = Path::new(config::AS_PATH).join(matrix_name);
        as_path.set_extension("mtx");
        let bs_path = Path::new(config::BS_PATH).join(matrix_name);

        let as_content = fs::read_to_string(&as_path).ok()?;
        // let as_content = fs::read_to_string(&as_path).unwrap_or_else(|err| {
            // return None;
            // panic!("Erro ao ler o arquivo A ({:?}): {err}", as_path);
            // panic!("Erro ao ler o arquivo A ({:?}): {err}", as_path);
        // });

        let sas = read::extract_sas(&as_content)?;
        let n = sas.len();

        // let opt_bs_content = fs::read_to_string(&bs_path);
        let sbs = read::handle_bs(bs_path, n);
        Some(Self { sas, sbs, slen: n })
    }

    pub unsafe fn initilize_xs(&self) -> Vec<f64> {
        let len = self.slen;
        let mut xs: Vec<f64> = Vec::with_capacity(len);
        let spare = xs.spare_capacity_mut();

        unsafe {
            for i in 0..len {
                let bi = *self.sbs.get_unchecked(i);
                let aii = *self.sas.get_unchecked(i, i);
                spare.get_unchecked_mut(i).write(bi / aii);
            }
        }
        unsafe {
            xs.set_len(len);
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

    // apenas para blocos contiguos
    pub unsafe fn jaspiom_step(
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

                // old
                for j in 0..start_row {
                    sum -= *self.sas.get_unchecked(i, j)
                        * *old_xs.get_unchecked(j);
                }

                // new
                for j in start_row..i {
                    sum -= *self.sas.get_unchecked(i, j)
                        * *chunk.get_unchecked(j - start_row);
                }

                // old
                for j in (i + 1)..len {
                    sum -= *self.sas.get_unchecked(i, j)
                        * *old_xs.get_unchecked(j);
                }


                // global to local
                *chunk.get_unchecked_mut(i - start_row) = sum / aii;
            }
        }
    }
    
    pub unsafe fn jaspiom_steps(
        &self,
        num_steps: usize,
        start_row: usize,
        end_row: usize,
        old_xs: &[f64],
        chunk: &mut [f64],
    ) {
        let len = self.slen;
        debug_assert!(end_row <= len, "end_row > matrix len");
        debug_assert!(old_xs.len() == len, "wrong old_xs len");
        debug_assert!(chunk.len() == end_row - start_row, "wrong chunk len");

        let mut tresh_hold = vec![0.; end_row - start_row];
        for i in start_row..end_row {
            for j in 0..start_row { unsafe {
                *tresh_hold.get_unchecked_mut(i - start_row) -= *self.sas.get_unchecked(i, j)
                        * *old_xs.get_unchecked(j); 
            } }

            for j in end_row..len { unsafe {
                *tresh_hold.get_unchecked_mut(i - start_row) -= *self.sas.get_unchecked(i, j)
                        * *old_xs.get_unchecked(j); 
            } }
        }


        for i in start_row..end_row {
            unsafe {
                let aii = *self.sas.get_unchecked(i, i);
                let mut sum = *self.sbs.get_unchecked(i) + *tresh_hold.get_unchecked(i - start_row);

                // new
                for j in start_row..i {
                    sum -= *self.sas.get_unchecked(i, j)
                        * *chunk.get_unchecked(j - start_row);
                }

                // old
                for j in (i+1)..end_row {
                    sum -= *self.sas.get_unchecked(i, j)
                        * *old_xs.get_unchecked(j);
                }



                // global to local
                *chunk.get_unchecked_mut(i - start_row) = sum / aii;
            }
        }

        for _ in 0..num_steps-1 {
            for i in start_row..end_row {
                unsafe {
                    let aii = *self.sas.get_unchecked(i, i);
                    let mut sum = *self.sbs.get_unchecked(i) + *tresh_hold.get_unchecked(i - start_row);
                    for j in start_row..end_row {
                        sum -= *self.sas.get_unchecked(i, j)
                            * *chunk.get_unchecked(j - start_row);
                    }
                    sum += aii * *chunk.get_unchecked(i - start_row);

                    // global to local
                    *chunk.get_unchecked_mut(i - start_row) = sum / aii;
                }
            }
        }
    }
    

    /// Calcula a Norma L2 (Euclidiana) do resíduo: ||b - Ax||_2
    pub fn calculate_residual(&self, xs: &[f64]) -> f64 {
        let len = self.slen;
        debug_assert!(
            xs.len() == len,
            "O tamanho do vetor x não corresponde ao sistema"
        );

        let mut residual_norm_sq = 0.0;

        for i in 0..len {
            unsafe {
                let mut ax_i = 0.0;

                // Produto escalar da linha i da matriz A pelo vetor x
                for j in 0..len {
                    ax_i += *self.sas.get_unchecked(i, j) * *xs.get_unchecked(j);
                }

                // r_i = b_i - (A * x)_i
                let r_i = *self.sbs.get_unchecked(i) - ax_i;

                // Soma os quadrados dos resíduos
                residual_norm_sq += r_i * r_i;
            }
        }

        // Retorna a raiz quadrada da soma
        residual_norm_sq.sqrt()
    }
}
