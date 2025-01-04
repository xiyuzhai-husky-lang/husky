use super::{stack::VdBaseqHypothesisStack, *};
use crate::{session::VdBaseqSession, term::VdMirTermFld};
use floated_sequential::db::FloaterDb;

pub struct VdBaseqHypothesisConstructor<'db, 'sess> {
    session: &'sess VdBaseqSession<'db>,
    stack: VdBaseqHypothesisStack<'sess>,
    arena: VdBaseqHypothesisArena<'sess>,
}

impl<'db, 'sess> VdBaseqHypothesisConstructor<'db, 'sess> {
    pub(crate) fn new(session: &'sess VdBaseqSession<'db>) -> Self {
        Self {
            session,
            stack: VdBaseqHypothesisStack::new(),
            arena: VdBaseqHypothesisArena::default(),
        }
    }
}

// # getters
impl<'db, 'sess> VdBaseqHypothesisConstructor<'db, 'sess> {
    pub fn stack(&self) -> &VdBaseqHypothesisStack<'sess> {
        &self.stack
    }

    pub fn arena(&self) -> &VdBaseqHypothesisArena<'sess> {
        &self.arena
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
        if let Some(hypothesis) = self.stack.get_active_hypothesis_with_expr(expr) {
            Some(hypothesis)
        } else if let Some(hypothesis) = self.stack.get_active_hypothesis_with_term(expr.term()) {
            todo!("allocate new hypothesis in stack");
            Some(hypothesis)
        } else {
            None
        }
    }

    pub(crate) fn construct_new_hypothesis(
        &mut self,
        expr: VdMirExprFld<'sess>,
        construction: VdBaseqHypothesisConstruction<'sess>,
    ) -> VdBaseqHypothesisIdx<'sess> {
        let hypothesis_idx = self
            .arena
            .alloc_one(VdBaseqHypothesisEntry { expr, construction });
        self.stack.append(hypothesis_idx, &self.arena);
        hypothesis_idx
    }
}
