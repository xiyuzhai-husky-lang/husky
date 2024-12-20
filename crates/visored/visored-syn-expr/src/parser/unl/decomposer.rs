//! The decomposer for uncontrolled natural language.
//!
//! It is highly parallelizable for efficient llm inference.
use crate::*;
use latex_ast::ast::rose::LxRoseAstIdxRange;
use sentence::VdSynSentenceIdx;
use std::sync::Mutex;

pub struct VdSynUnlDecomposer<'db> {
    pub(crate) builder: Mutex<VdSynExprBuilder<'db>>,
}

impl<'db> VdSynUnlDecomposer<'db> {
    pub fn new(builder: VdSynExprBuilder<'db>) -> Self {
        Self {
            builder: Mutex::new(builder),
        }
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn unl_decomposer(self) -> VdSynUnlDecomposer<'db> {
        VdSynUnlDecomposer::new(self)
    }
}

impl<'db> VdSynUnlDecomposer<'db> {
    pub fn decompose_sentences(mut self) -> VdSynSentenceIdx {
        todo!()
    }
}
