use super::*;
use crate::coercion::VdBsqCoercion;
use std::marker::PhantomData;
use visored_entity_path::theorem::VdTheoremPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdBsqHypothesisConstruction<'sess> {
    Sorry,
    Apply {
        path: VdTheoremPath,
        is_real_coercion: VdBsqCoercion<'sess>,
    },
    TermEquivalent {
        hypothesis: VdBsqHypothesisIdx<'sess>,
    },
    Assume,
    Phantom(PhantomData<&'sess ()>),
}
