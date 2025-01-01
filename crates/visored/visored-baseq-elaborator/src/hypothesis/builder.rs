use super::{stack::VdBaseqHypothesisStack, *};
use crate::term::VdMirTermFld;
use floated_sequential::db::FloaterDb;
use rustc_hash::FxHashMap;

pub struct VdBaseqHypothesisBuilder<'sess> {
    db: &'sess FloaterDb,
    stack: VdBaseqHypothesisStack<'sess>,
    arena: VdBaseqHypothesisArena<'sess>,
    expr_to_hypothesis: FxHashMap<VdMirExprFld<'sess>, VdBaseqHypothesisIdx<'sess>>,
    term_to_hypothesis: FxHashMap<VdMirTermFld<'sess>, VdBaseqHypothesisIdx<'sess>>,
}

impl<'sess> VdBaseqHypothesisBuilder<'sess> {
    pub(crate) fn new(db: &'sess FloaterDb) -> Self {
        Self {
            db,
            stack: VdBaseqHypothesisStack::new(),
            arena: VdBaseqHypothesisArena::default(),
            expr_to_hypothesis: FxHashMap::default(),
            term_to_hypothesis: FxHashMap::default(),
        }
    }
}

impl<'sess> VdBaseqHypothesisBuilder<'sess> {
    /// Attempts to find an existing hypothesis that matches the given expression.
    ///
    /// This method implements functionality similar to the `assumption` tactic in proof
    /// assistants like Lean and Coq. It searches for a matching hypothesis in the current
    /// context that could prove the given expression.
    ///
    /// If an existing hypothesis is found with the same expression, return it directly.
    ///
    /// Otherwise, if an existing hypothesis is found with the same term, return a new hypothesis derived from it.
    pub fn assumption(&mut self, expr: VdMirExprFld<'sess>) -> Option<VdBaseqHypothesisIdx<'sess>> {
        if let Some(&idx) = self.expr_to_hypothesis.get(&expr) {
            Some(idx)
        } else if let Some(&idx) = self.term_to_hypothesis.get(&expr.term(self.db)) {
            todo!("allocate new hypothesis in stack");
            Some(idx);
        } else {
            None
        }
    }
}
