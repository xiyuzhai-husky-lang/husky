pub mod assumption;
pub mod library_search;
pub mod ring;
pub mod term_trivial;

use crate::{
    elaborator::VdBsqElaboratorInner,
    expr::VdMirExprFld,
    hypothesis::{contradiction::VdBsqHypothesisResult, VdBsqHypothesisIdx},
};
use alt_option::AltOption;

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqTactic {
    TermTrivial,
    Assumption,
    LibrarySearch,
}

impl VdBsqTactic {
    pub fn run<'db, 'sess>(
        &self,
        prop: VdMirExprFld<'sess>,
        elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    ) -> VdBsqHypothesisResult<'sess, AltOption<VdBsqHypothesisIdx<'sess>>> {
        match self {
            VdBsqTactic::TermTrivial => elaborator.term_trivial(prop),
            VdBsqTactic::Assumption => elaborator.assumption(prop),
            VdBsqTactic::LibrarySearch => elaborator.library_search(prop),
        }
    }
}
