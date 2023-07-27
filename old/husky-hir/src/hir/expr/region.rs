use super::*;

#[salsa::tracked(db = HirDb, jar = HirJar)]
pub struct ExprHirRegion {}

#[derive(Debug, PartialEq, Eq)]
pub struct TrackedExprHirRegion {
    region: ExprHirRegion,
    tys: ExprHirTypeRegion,
    srcs: ExprHirSourceRegion,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExprHirTypeRegion {}

#[derive(Debug, PartialEq, Eq)]
pub struct ExprHirSourceRegion {}
