//! `unl` stands for `uncontrolled natural language`.
//! It's processed by the grammar of `snl`, super natural language.
use crate::*;
use latex_ast::ast::rose::LxRoseAstIdxRange;
use sentence::VdSynSentenceIdx;

pub struct VdSynUnlParser<'a, 'db> {
    builder: &'a mut VdSynExprBuilder<'db>,
}

impl<'a, 'db> VdSynUnlParser<'a, 'db> {
    pub fn new(builder: &'a mut VdSynExprBuilder<'db>) -> Self {
        Self { builder }
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn unl_parser(&mut self) -> VdSynUnlParser<'_, 'db> {
        VdSynUnlParser::new(self)
    }
}

impl<'a, 'db> VdSynUnlParser<'a, 'db> {
    pub fn parse_sentence_from_rose_asts(mut self, asts: LxRoseAstIdxRange) -> VdSynSentenceIdx {
        todo!()
    }
}
