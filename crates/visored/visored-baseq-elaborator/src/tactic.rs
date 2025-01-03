pub mod library_search;
pub mod ring;

use crate::{
    elaborator::VdBaseqElaboratorInner,
    expr::VdMirExprFld,
    hypothesis::{contradiction::VdBaseqHypothesisResult, VdBaseqHypothesisIdx},
};
use alt_option::AltOption;

#[derive(Debug, PartialEq, Eq)]
pub enum VdBaseqTactic {
    LibrarySearch,
}

impl VdBaseqTactic {
    pub fn run<'db, 'sess>(
        &self,
        prop: VdMirExprFld<'sess>,
        elaborator: &mut VdBaseqElaboratorInner<'db, 'sess>,
    ) -> VdBaseqHypothesisResult<'sess, AltOption<VdBaseqHypothesisIdx<'sess>>> {
        match self {
            VdBaseqTactic::LibrarySearch => elaborator.library_search(prop),
        }
    }
}
