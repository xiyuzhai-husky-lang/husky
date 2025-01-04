pub mod library_search;
pub mod ring;

use crate::{
    elaborator::VdBsqElaboratorInner,
    expr::VdMirExprFld,
    hypothesis::{contradiction::VdBsqHypothesisResult, VdBsqHypothesisIdx},
};
use alt_option::AltOption;

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqTactic {
    LibrarySearch,
}

impl VdBsqTactic {
    pub fn run<'db, 'sess>(
        &self,
        prop: VdMirExprFld<'sess>,
        elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    ) -> VdBsqHypothesisResult<'sess, AltOption<VdBsqHypothesisIdx<'sess>>> {
        match self {
            VdBsqTactic::LibrarySearch => elaborator.library_search(prop),
        }
    }
}
