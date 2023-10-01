mod eager_call;
mod eager_expr;
mod eager_stmt;
mod lazy_call;
mod lazy_expr;
mod lazy_stmt;
mod loop_group;
mod val_item;

pub use self::eager_call::*;
pub use self::eager_expr::*;
pub use self::eager_stmt::*;
pub use self::lazy_call::*;
pub use self::lazy_expr::*;
pub use self::lazy_stmt::*;
pub use self::loop_group::*;
pub use self::val_item::*;

use crate::*;
use husky_sema_expr::SemaExprIdx;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = TraceDb)]
pub enum Trace {
    LazyCall(LazyCallTrace),
    LazyExpr(LazyExprTrace),
    LazyStmt(LazyStmtTrace),
    EagerCall(EagerCallTrace),
    EagerExpr(EagerExprTrace),
    EagerStmt(EagerStmtTrace),
}

pub enum AssociatedExprTraces<'a> {
    Eager(&'a [(SemaExprIdx, EagerExprTrace)]),
    Lazy(&'a [(SemaExprIdx, LazyExprTrace)]),
}

#[test]
fn associated_expr_traces_size() {
    assert_eq!(
        std::mem::size_of::<Option<AssociatedExprTraces>>(),
        std::mem::size_of::<AssociatedExprTraces>()
    )
}

impl Trace {
    pub fn associated_expr_traces(self, db: &dyn TraceDb) -> Option<AssociatedExprTraces> {
        match self {
            Trace::LazyCall(_) => None,
            Trace::LazyExpr(_) => None,
            Trace::LazyStmt(trace) => {
                Some(AssociatedExprTraces::Lazy(trace.associated_expr_traces(db)))
            }
            Trace::EagerCall(_) => None,
            Trace::EagerExpr(_) => None,
            Trace::EagerStmt(trace) => Some(AssociatedExprTraces::Eager(
                trace.associated_expr_traces(db),
            )),
        }
    }
}
