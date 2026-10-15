/*

All of the methods uses a serial checkup!

*/

mod classical;
mod delayed;
mod gapped;

pub use classical::method as jacobi_classical;
pub use delayed::method as jacobi_delayed;
pub use gapped::method as jacobi_gapped;



