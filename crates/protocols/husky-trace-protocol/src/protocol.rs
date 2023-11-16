#[cfg(feature = "mock")]
pub mod mock;
#[cfg(feature = "trivial")]
pub mod trivial;

use crate::*;

pub trait IsTraceProtocol:
    Default
    + std::fmt::Debug
    + Clone
    + PartialEq
    + Eq
    + Serialize
    + for<'a> Deserialize<'a>
    + Send
    + 'static
{
    type VisualProtocol: IsVisualProtocol;
}

pub trait IsTrace: std::fmt::Debug + Eq + Copy + From<TraceId> + Into<TraceId> {}

pub enum TraceKindProtocol {
    LazyCall,
    LazyExpr,
    LazyStmt,
    EagerCall,
    EagerExpr,
    EagerStmt,
}
