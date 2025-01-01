use super::*;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VdBaseqHypothesisConstruction<'sess> {
    Phantom(PhantomData<&'sess ()>),
}
