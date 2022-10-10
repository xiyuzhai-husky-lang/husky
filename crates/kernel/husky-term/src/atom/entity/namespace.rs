use optional::{Noned, OptEq};

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TermNamespace(TermPtr);

impl Noned for TermNamespace {
    fn is_none(&self) -> bool {
        self.0.is_none()
    }

    fn get_none() -> Self {
        Self(TermPtr::get_none())
    }
}

impl OptEq for TermNamespace {
    fn opt_eq(&self, other: &Self) -> bool {
        self.0.opt_eq(&other.0)
    }
}
