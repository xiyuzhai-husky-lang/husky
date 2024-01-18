use crate::*;

pub trait IsTraceProtocol:
    Default + std::fmt::Debug + Clone + PartialEq + Eq + Send + 'static
{
    type Pedestal: IsPedestal;
    type Figure: IsFigure;
}

impl IsTraceProtocol for () {
    type Pedestal = ();

    type Figure = ();
}

pub trait IsPedestal:
    std::fmt::Debug
    + Default
    + PartialEq
    + Eq
    + Clone
    + Copy
    + Send
    + Sync
    + Serialize
    + std::hash::Hash
    + for<'a> Deserialize<'a>
    + 'static
{
}

impl IsPedestal for () {}

pub trait IsTraceProtocolFull: IsTraceProtocol + Serialize + for<'a> Deserialize<'a> {}

impl<T> IsTraceProtocolFull for T where T: IsTraceProtocol + Serialize + for<'a> Deserialize<'a> {}

pub trait IsTrace: std::fmt::Debug + Eq + Copy + From<TraceId> + Into<TraceId> {}

pub enum TraceKindProtocol {
    LazyCall,
    LazyExpr,
    LazyStmt,
    EagerCall,
    EagerExpr,
    EagerStmt,
}
