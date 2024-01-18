use std::num::NonZeroU32;
use vec_like::OrderedSmallVecSet;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TraceId {
    value: NonZeroU32,
}

pub type AccompanyingTraceIds = OrderedSmallVecSet<TraceId, 4>;

impl TraceId {
    pub fn new(raw: NonZeroU32) -> Self {
        Self { value: raw }
    }

    pub fn from_index(index: u32) -> Self {
        Self {
            value: unsafe { NonZeroU32::new_unchecked(index + 1) },
        }
    }

    pub fn value(&self) -> NonZeroU32 {
        self.value
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
