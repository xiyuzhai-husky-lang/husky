mod eager_call;
mod eager_expr;
mod eager_stmt;
mod lazy_call;
mod lazy_expr;
mod lazy_stmt;
mod loop_group;
mod submodule;
mod val_item;

pub use self::eager_call::*;
pub use self::eager_expr::*;
pub use self::eager_stmt::*;
pub use self::lazy_call::*;
pub use self::lazy_expr::*;
pub use self::lazy_stmt::*;
pub use self::loop_group::*;
pub use self::submodule::*;
pub use self::val_item::*;

use crate::*;
use husky_sema_expr::SemaExprIdx;
use husky_trace_protocol::settings::TraceSettings;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = TraceDb)]
pub enum Trace {
    Module(SubmoduleTrace),
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

pub enum Subtraces<'a> {
    Submodule(&'a [SubmoduleSubtrace]),
    LazyCall(&'a [LazyCallSubtrace]),
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
            Trace::Module(_) => None,
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

    pub fn subtraces<'a>(
        self,
        settings: &TraceSettings,
        db: &'a dyn TraceDb,
    ) -> Option<Subtraces<'a>> {
        match self {
            Trace::Module(slf) => slf.subtraces(db).map(Subtraces::Submodule),
            Trace::LazyCall(slf) => slf.subtraces(db).map(Subtraces::LazyCall),
            Trace::LazyExpr(slf) => todo!(),
            Trace::LazyStmt(slf) => todo!(),
            Trace::EagerCall(slf) => todo!(),
            Trace::EagerExpr(slf) => todo!(),
            Trace::EagerStmt(slf) => todo!(),
        }
    }
}
