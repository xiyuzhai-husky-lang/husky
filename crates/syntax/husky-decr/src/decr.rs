mod derive;

pub use derive::*;

use crate::*;

pub enum Decr {
    Derive(DeriveDecr),
}
