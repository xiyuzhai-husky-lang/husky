mod eager_call;
mod eager_expr;
mod eager_stmt;
mod lazy_call;
mod lazy_expr;
mod lazy_stmt;
mod val_item;

pub use self::eager_call::*;
pub use self::eager_expr::*;
pub use self::eager_stmt::*;
pub use self::lazy_call::*;
pub use self::lazy_expr::*;
pub use self::lazy_stmt::*;
pub use self::val_item::*;

use crate::*;
use husky_coword::Ident;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TracePath {
    ValItem(ValItemTracePath),
    LazyCall(LazyCallTracePath),
    LazyExpr(LazyExprTracePath),
    LazyStmt(LazyStmtTracePath),
    EagerExpr(EagerExprTracePath),
    EagerStmt(EagerStmtTracePath),
}
// parent: Option<TracePath>,
// data: TracePathData,
// disambiguator: u8,

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TracePathData {
    Val { path: FugitivePath },
    LazyStmt(LazyStmtTracePathData),
    LazyExpr {},
    EagerStmt {},
    EagerExpr {},
}
