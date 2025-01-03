use super::*;
use std::marker::PhantomData;
use visored_entity_path::theorem::VdTheoremPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VdBaseqHypothesisConstruction<'sess> {
    Sorry,
    Apply { path: VdTheoremPath },
    Phantom(PhantomData<&'sess ()>),
}
