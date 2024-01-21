use shifted_unsigned_int::ShiftedU32;
use std::num::NonZeroU32;
use vec_like::OrderedSmallVecSet;

use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TraceId(ShiftedU32);

impl std::fmt::Debug for TraceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index: usize = self.0.into();
        index.fmt(f)
    }
}

pub type AccompanyingTraceIds = OrderedSmallVecSet<TraceId, 4>;

impl TraceId {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceKind {
    Submodule,
    ValItem,
    LazyCall,
    LazyCallInput,
    LazyExpr,
    LazyPatternExpr,
    LazyStmt,
    EagerCall,
    EagerExpr,
    EagerPatternExpr,
    EagerStmt,
    EagerCallInput,
}
