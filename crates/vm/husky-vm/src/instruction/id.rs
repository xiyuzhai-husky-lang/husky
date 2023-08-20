use std::{
    panic::RefUnwindSafe,
    sync::atomic::{AtomicUsize, Ordering},
};

use husky_hir_eager_expr::{HirEagerExprIdx, HirEagerStmtIdx};
use husky_text::HasSourceRange;

static NEXT_VM_INSTRUCTION_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionId(pub(crate) usize);

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum InstructionSource {
    Expr(HirEagerExprIdx),
    Stmt(HirEagerStmtIdx),
}

impl Default for InstructionId {
    fn default() -> Self {
        let raw = NEXT_VM_INSTRUCTION_ID.fetch_add(1, Ordering::Relaxed);
        Self(raw)
    }
}
