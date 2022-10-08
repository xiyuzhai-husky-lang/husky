mod error;

use crate::TermPtr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Ty(TermPtr);
