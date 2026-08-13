// AI Generated

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 1. Avisa o compilador: "Se mexerem nessa pasta, recompile o projeto automaticamente!"
    println!("cargo:rerun-if-changed=data/");

    let mut output = String::from("pub const MATRIZES: &[MatrixInfo] = &[\n");

    // 2. O build.rs faz o trabalho sujo de I/O enquanto você ainda está compilando
    if let Ok(entradas) = fs::read_dir("data/as/") {
        for entrada in entradas.flatten() {
            let path = entrada.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "mtx") {
                let name = path.file_stem().unwrap().to_str().unwrap();

                // Lê o texto gigante pra memória do compilador
                let content = fs::read_to_string(&path).unwrap();
                let mut lines = content.lines();

                // Extrai as infos
                let fst_line = lines.next().unwrap();
                let is_symmetric = fst_line.contains("symmetric");

                let snd_line = lines.next().unwrap();
                let size: usize = snd_line
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                // 3. Escreve a struct hardcoded na string!
                // Os bytes gigantescos nunca vão pro executável.
                output.push_str(&format!(
                    "    MatrixInfo {{ name: \"{name}\", len: {size}, is_symmetric: {is_symmetric} }},\n"
                ));
            }
        }
    }

    output.push_str("];\n");

    // 4. Salva o código gerado numa pasta temporária do compilador
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("matrizes_hardcoded.rs");
    fs::write(&dest_path, output).unwrap();
}
