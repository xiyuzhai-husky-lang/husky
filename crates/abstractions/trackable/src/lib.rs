#![feature(try_trait_v2)]

mod atom;
mod map;
mod vec;

use std::marker::PhantomData;

pub use self::atom::*;
pub use self::map::*;
pub use self::vec::*;

pub trait Trackable: Sized {
    type Change;
    fn take_change(&mut self) -> Self::Change;
}
