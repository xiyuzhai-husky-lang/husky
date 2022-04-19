#![allow(dead_code, warnings)]
#[cfg(test)] // this crate is for trying stuffs
mod try_atomic;
mod try_control_flow;
mod try_random;
mod try_rayon;
mod try_ref;
#[cfg(test)]
mod try_salsa;
mod try_serde;
mod try_trait;

use check_utils::*;
use print_utils::*;
