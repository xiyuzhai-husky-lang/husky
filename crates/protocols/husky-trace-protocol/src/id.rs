use std::num::NonZeroU32;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraceId {
    kind: TraceKind,
    value: NonZeroU32,
}

impl TraceId {
    pub fn new(kind: TraceKind, raw: NonZeroU32) -> Self {
        Self { kind, value: raw }
    }

    pub fn kind(&self) -> TraceKind {
        self.kind
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Vec<TraceId> {
//     start: TraceId,
//     end: TraceId,
// }

// impl Vec<TraceId> {
//     pub(crate) fn new(start: TraceId, end: TraceId) -> Self {
//         Self { start, end }
//     }

//     #[cfg(feature = "mock")]
//     pub(crate) fn new_mock(start: usize, end: usize) -> Self {
//         Self {
//             start: TraceId::from_index(start),
//             end: TraceId::from_index(end),
//         }
//     }

//     pub fn start(&self) -> TraceId {
//         self.start
//     }

//     pub fn end(&self) -> TraceId {
//         self.end
//     }

//     pub fn into_iter(self) -> impl Iterator<Item = TraceId> {
//         (self.start.index()..self.end.index())
//             .into_iter()
//             .map(|index| TraceId::from_index(index))
//     }
// }
