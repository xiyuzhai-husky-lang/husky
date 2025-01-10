pub mod assumption;
pub mod comm_ring;
pub mod library_search;
pub mod term_trivial;

use crate::{
    elaborator::VdBsqElaboratorInner,
    expr::VdBsqExprFld,
    hypothesis::{contradiction::VdBsqHypothesisResult, VdBsqHypothesisIdx},
    *,
};
use alt_option::AltOption;
use miracle::HasMiracleFull;

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqTactic {
    Assumption,
    TermTrivial,
    LibrarySearch,
    CommRing,
}

// Trivial tactics are not tracked
#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqTacticCall {
    LibrarySearch,
    CommRing,
}

impl VdBsqTactic {
    pub fn run<'db, 'sess>(
        &self,
        prop: VdBsqExprFld<'sess>,
        elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    ) -> Mhr<'sess> {
        match self {
            VdBsqTactic::Assumption => elaborator.assumption(prop),
            VdBsqTactic::TermTrivial => elaborator.term_trivial(prop),
            VdBsqTactic::LibrarySearch => elaborator.library_search(prop),
            VdBsqTactic::CommRing => elaborator.comm_ring(prop),
        }
    }
}
