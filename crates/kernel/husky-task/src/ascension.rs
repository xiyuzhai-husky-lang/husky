use crate::*;
use husky_linkage::db::LinkageDb;
use husky_trace_protocol::protocol::IsTraceProtocol;
use husky_vfs::VfsDb;

pub trait IsDevAscension {
    type Base: 'static;
    type ComptimeDb: Default + VfsDb + LinkageDb;
    type Linktime: IsLinktime<Self::ComptimeDb>;
    type Value;
    type RuntimeStorage: Default + Send;
    type RuntimeSpecificConfig: Default + Send;
    type TraceProtocol: IsTraceProtocol;
}
