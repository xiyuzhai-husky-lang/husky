#![feature(try_trait_v2)]

mod proj_map;
mod updated_m;
mod updating_m;

use std::marker::PhantomData;

pub use proj_map::*;
pub use updated_m::ProjUpdatedR;
pub use updating_m::ProjUpdatingR;

use updated_m::*;
use updating_m::*;
