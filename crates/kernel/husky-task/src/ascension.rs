use crate::*;
use husky_trace_protocol::protocol::IsTraceProtocol;
use husky_vfs::VfsDb;

pub trait IsDevAscension {
    type Base: 'static;
    type Linktime: IsLinktime;
    type Value;
    type RuntimeStorage: Default + Send;
    type RuntimeSpecificConfig: Default + Send;
    type TraceProtocol: IsTraceProtocol;
}
