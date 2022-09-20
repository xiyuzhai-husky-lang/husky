#![feature(try_trait_v2)]

mod proj_atom;
mod proj_map;
mod proj_set;
mod proj_vec;
mod updated_m;
mod updating_m;

use std::marker::PhantomData;

pub use proj_atom::*;
pub use proj_map::*;
pub use proj_set::*;
pub use proj_vec::*;
pub use updated_m::ProjUpdatedR;
pub use updating_m::ProjUpdatingR;

use updated_m::*;
use updating_m::*;
