use crate::*;

use latex_ast::ast::rose::LxRoseAstIdxRange;

use sentence::VdSynSentenceIdx;

pub struct VdSynUnlEntangler<'a, 'db> {
    pub(crate) builder: &'a mut VdSynExprBuilder<'db>,
}

impl<'a, 'db> VdSynUnlEntangler<'a, 'db> {
    pub fn new(builder: &'a mut VdSynExprBuilder<'db>) -> Self {
        Self { builder }
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn unl_entangler(&mut self) -> VdSynUnlEntangler<'_, 'db> {
        VdSynUnlEntangler::new(self)
    }
}

impl<'a, 'db> VdSynUnlEntangler<'a, 'db> {
    pub fn entangle_sentence_from_rose_asts(mut self, asts: LxRoseAstIdxRange) -> VdSynSentenceIdx {
        todo!()
    }
}
