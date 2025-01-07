use super::*;
use crate::term::VdBsqTerm;
use floated_sequential::db::FloaterDb;
use rustc_hash::FxHashMap;

/// A stack of baseq hypotheses.
///
/// This structure maintains a stack of hypotheses and maps that associate expressions and terms
/// with hypothesis records. The maps may contain outdated information, so all lookups are validated
/// against the current state of the hypothesis stack.
///
/// # Implementation Strategy
/// The maps are intentionally allowed to contain outdated entries as an optimization technique.
/// When hypotheses are removed from the stack (via rollback), we avoid the cost of updating
/// the maps immediately. Instead, we keep the stale entries and validate them at lookup time.
/// This trades slightly slower lookups for much faster rollback operations, which is beneficial
/// when the stack is frequently rolled back.
///
/// # Maps and Validation
/// - `expr_to_hypothesis`: Maps expressions to hypothesis records, but entries may be stale
/// - `term_to_hypothesis`: Maps terms to hypothesis records, but entries may be stale
///
/// Both maps are validated during lookups by checking if the recorded hypothesis still exists
/// at the expected position in the stack. This ensures we only return valid, "live" hypotheses.
pub struct VdBsqHypothesisStack<'sess> {
    active_hypotheses: Vec<VdBsqHypothesisIdx<'sess>>,
    expr_to_hypothesis_map: FxHashMap<VdMirExprFld<'sess>, VdBsqHypothesisRecord<'sess>>,
    term_to_hypothesis_map: FxHashMap<VdBsqTerm<'sess>, VdBsqHypothesisRecord<'sess>>,
}

#[derive(Debug, Clone, Copy)]
pub struct VdBsqHypothesisRecord<'sess> {
    stack_idx: usize,
    hypothesis_idx: VdBsqHypothesisIdx<'sess>,
}

impl<'sess> VdBsqHypothesisStack<'sess> {
    pub(super) fn new() -> Self {
        Self {
            active_hypotheses: Vec::new(),
            expr_to_hypothesis_map: FxHashMap::default(),
            term_to_hypothesis_map: FxHashMap::default(),
        }
    }
}

impl<'sess> VdBsqHypothesisStack<'sess> {
    pub fn len(&self) -> usize {
        self.active_hypotheses.len()
    }

    pub(crate) fn get_active_hypothesis_with_expr(
        &self,
        expr: VdMirExprFld<'sess>,
    ) -> Option<VdBsqHypothesisIdx<'sess>> {
        let record = self.expr_to_hypothesis_map.get(&expr).copied()?;
        (self.active_hypotheses.get(record.stack_idx) == Some(&record.hypothesis_idx))
            .then_some(record.hypothesis_idx)
    }

    pub(crate) fn get_active_hypothesis_with_term(
        &self,
        term: VdBsqTerm<'sess>,
    ) -> Option<VdBsqHypothesisIdx<'sess>> {
        let record = self.term_to_hypothesis_map.get(&term).copied()?;
        (self.active_hypotheses.get(record.stack_idx) == Some(&record.hypothesis_idx))
            .then_some(record.hypothesis_idx)
    }
}

impl<'sess> VdBsqHypothesisStack<'sess> {
    pub fn append(
        &mut self,
        hypothesis_idx: VdBsqHypothesisIdx<'sess>,
        arena: &VdBsqHypothesisArena<'sess>,
    ) {
        let stack_idx = self.active_hypotheses.len();
        self.active_hypotheses.push(hypothesis_idx);
        let expr = arena[hypothesis_idx].expr();
        let term = expr.term();
        // never recreate an active hypothesis with the exact same expression
        debug_assert!(
            self.get_active_hypothesis_with_expr(expr).is_none(),
            "hypothesis already exists, expr: {:?}",
            expr
        );
        self.expr_to_hypothesis_map.insert(
            expr,
            VdBsqHypothesisRecord {
                stack_idx,
                hypothesis_idx,
            },
        );
        // only add the hypothesis to the term map if the term is not already present
        if self.get_active_hypothesis_with_term(term).is_none() {
            self.term_to_hypothesis_map.insert(
                term,
                VdBsqHypothesisRecord {
                    stack_idx,
                    hypothesis_idx,
                },
            );
        }
    }

    pub fn rollback(&mut self, len: usize) {
        debug_assert!(len <= self.len());
        self.active_hypotheses.truncate(len);
    }
}
