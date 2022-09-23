#![feature(try_trait_v2)]

mod atom;
mod map;
mod monads;
mod set;
mod vec;

use std::marker::PhantomData;

pub use atom::*;
pub use map::*;
pub use monads::*;
pub use set::*;
pub use vec::*;

pub trait Trackable: Sized {
    type Change;
    fn take_change(&mut self) -> TrackableTakeChangeM<Self>;
}
