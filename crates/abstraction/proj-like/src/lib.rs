#![feature(try_trait_v2)]

mod proj_atom;
mod proj_map;
mod proj_monads;
mod proj_set;
mod proj_vec;

use std::marker::PhantomData;

pub use proj_atom::*;
pub use proj_map::*;
pub use proj_monads::*;
pub use proj_set::*;
pub use proj_vec::*;

pub trait Proj: Sized {
    type Change;
    fn take_change(&mut self) -> ProjTakeChangeM<Self>;
}
