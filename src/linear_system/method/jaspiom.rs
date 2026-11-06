// use crate::experiment::
use crate::linear_system::LinearSystem;
use std::mem;
use std::thread;

use crate::linear_system::checkup::classical_serial;

use crate::config;
// use crate::linear_system::LinearSystem;
// use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
// use std::thread; // Para acessar config::EPSI




mod classical     ;
mod gap_in_even   ;
mod gap_out_even  ;
mod gap_mixed_even;
