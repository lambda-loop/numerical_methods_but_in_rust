
mod target;

use std::env;
use std::fs::{read_dir};

use crate::config::{AS_PATH, LONG_AS_PATH};

pub fn setup(
    target_path: &str
) -> std::io::Result<()> {
    let mut ms      = read_dir(AS_PATH)?;
    let mut long_ms = read_dir(LONG_AS_PATH)?;

    // let mut target_tabble = String::new();

    for m in ms.chain(long_ms) {
        println!("hey");
        let entry = m?;
        if !entry.file_type()?.is_file() {
            continue
        }

        if let Some(ext) = entry.path().extension() {
            if ext != ".mtx" { continue }
            let file_name = entry.file_name();
            println!("{file_name:?}");
        }
    }

    Ok(())
}
