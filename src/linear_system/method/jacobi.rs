/*

All of the methods uses a serial checkup!

*/

mod classical;
mod delayed;
mod galayed;
mod gapped;

pub use classical::method as classical;
pub use delayed::method as delayed;
pub use galayed::method as galayed;
pub use gapped::method as gapped;



