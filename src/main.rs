//
mod config;
mod experiment;
mod linear_system;

use linear_system::LinearSystem;
fn foo() {}

fn main() {
    let ls = LinearSystem::new("add32");
    println!("{ls:?}");
}
