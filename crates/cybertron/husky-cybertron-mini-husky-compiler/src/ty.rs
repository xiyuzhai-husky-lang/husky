pub mod checking;
pub mod expectation;
pub mod signature;

use crate::token::ident::Ident;
use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Ident(Ident),
    Option(Ident),
    Vec(Ident),
}
