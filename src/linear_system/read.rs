//

use super::matrix::SquareMatrix;

use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn handle_bs(bs_path: PathBuf, n: usize) -> Vec<f64> {
    let opt_bs_content = fs::read_to_string(&bs_path);
    match opt_bs_content {
        Ok(content) => extract_sbs(&content),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => create_and_get_bs(bs_path, n),
            _ => panic!("problem with bs o bs_path: {bs_path:?} \n"),
        },
    }
}

pub fn create_and_get_bs(bs_path: PathBuf, n: usize) -> Vec<f64> {
    let random_numbers: Vec<i32> = (0..n).map(|_| rand::random_range(1..=100)).collect();

    let bs_content: String = random_numbers.iter().map(|x| format!("{x}\n")).collect();
    fs::write(bs_path, bs_content).unwrap();

    random_numbers.iter().map(|x| *x as f64).collect()
}

pub fn extract_sbs(input: &str) -> Vec<f64> {
    input.lines().map(|s| s.parse::<f64>().unwrap()).collect()
    // let mut out_bs: vec<f64> = vec::with_capacity(n);

    // unsafe {
    //     for (i, val) in input
    //         .lines()
    //         .map(|s| s.parse::<f64>())
    //         .map(|res| res.unwrap())
    //         .enumerate()
    //     {
    //         *out_bs.get_unchecked_mut(i) = val;
    //     }
    // }

    // out_bs
}

// where "sas" stands for system 'as
pub fn extract_sas(input: &str) -> Option<SquareMatrix> {
    let mut lines = input.split('\n');

    let is_symmetric = lines.next().unwrap().trim().contains("symmetric");
    let n = lines.next()?;
    let n = parse_header(n)?;
    let mut matrix = SquareMatrix::new(n);

    for line in lines {
        let mut tokens = line.trim().split_ascii_whitespace();
        let Some(row) = tokens.next().and_then(|s| s.parse::<usize>().ok()) else {
            break;
        };
        let col = tokens.next()?.parse::<usize>().ok()?;
        let val = match tokens.next() {
            Some(val_str) => {
                let Ok(val) = val_str.parse::<f64>() else {
                    return None;
                };
                val
            },
            None => 1.0,
        };

        let i = row - 1;
        let j = col - 1;
        unsafe {
            *matrix.get_unchecked_mut(i, j) = val;
            if is_symmetric {
                *matrix.get_unchecked_mut(j, i) = val;
            }
        }
    }

    Some(matrix)
}

fn parse_header(header: &str) -> Option<usize> {
    let mut parts = header
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let rows = parts.next().unwrap();
    let cols = parts.next().unwrap();

    if rows == cols {
        Some(rows)
    } else {
        None
    }
}
