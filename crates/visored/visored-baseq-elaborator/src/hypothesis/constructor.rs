use super::{stack::VdBaseqHypothesisStack, *};
use crate::{session::VdBaseqSession, term::VdMirTermFld};
use floated_sequential::db::FloaterDb;
use rustc_hash::FxHashMap;

pub struct VdBaseqHypothesisConstructor<'db, 'sess> {
    session: &'sess VdBaseqSession<'db>,
    stack: VdBaseqHypothesisStack<'sess>,
    arena: VdBaseqHypothesisArena<'sess>,
    expr_to_hypothesis: FxHashMap<VdMirExprFld<'sess>, VdBaseqHypothesisIdx<'sess>>,
    term_to_hypothesis: FxHashMap<VdMirTermFld<'sess>, VdBaseqHypothesisIdx<'sess>>,
}

impl<'db, 'sess> VdBaseqHypothesisConstructor<'db, 'sess> {
    pub(crate) fn new(session: &'sess VdBaseqSession<'db>) -> Self {
        Self {
            session,
            stack: VdBaseqHypothesisStack::new(),
            arena: VdBaseqHypothesisArena::default(),
            expr_to_hypothesis: FxHashMap::default(),
            term_to_hypothesis: FxHashMap::default(),
        }
    }
}

impl<'db, 'sess> VdBaseqHypothesisConstructor<'db, 'sess> {
    /// Attempts to find an existing hypothesis that matches the given expression.
    ///
    /// This method implements functionality similar to the `assumption` tactic in proof
    /// assistants like Lean and Coq. It searches for a matching hypothesis in the current
    /// context that could prove the given expression.
    ///
    /// If an existing hypothesis is found with the same expression, return it directly.
    ///
    /// Otherwise, if an existing hypothesis is found with the same term, return a new hypothesis derived from it.
    pub(crate) fn assumption(
        &mut self,
        expr: VdMirExprFld<'sess>,
    ) -> Option<VdBaseqHypothesisIdx<'sess>> {
        if let Some(&idx) = self.expr_to_hypothesis.get(&expr) {
            Some(idx)
        } else if let Some(&idx) = self
            .term_to_hypothesis
            .get(&expr.term(self.session.floater_db()))
        {
            todo!("allocate new hypothesis in stack");
            Some(idx);
        } else {
            None
        }
    }

    pub(crate) fn construct_new_hypothesis(
        &mut self,
        expr: VdMirExprFld<'sess>,
        construction: VdBaseqHypothesisConstruction<'sess>,
    ) -> VdBaseqHypothesisIdx<'sess> {
        todo!()
    }
}
