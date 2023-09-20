mod eager_expr;
mod eager_stmt;
mod item;
mod lazy_expr;
mod lazy_stmt;

use self::eager_expr::*;
use self::eager_stmt::*;
use self::lazy_expr::*;
use self::lazy_stmt::*;
use crate::*;
use husky_coword::Ident;

#[salsa::interned(db = TracePathDb, jar = TracePathJar)]
pub struct TracePath {
    parent: Option<TracePath>,
    data: TracePathData,
    disambiguator: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TracePathData {
    Val { path: FugitivePath },
    LazyStmt(LazyStmtTracePathData),
    LazyExpr {},
    EagerStmt {},
    EagerExpr {},
}
