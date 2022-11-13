mod old;

pub use old::*;

use crate::*;

pub type DebugtimeState = ServerTraceState<TraceNode>;

pub type DebugtimeOldState = ServerTraceOldState<TraceNode>;

pub type DebugtimeStateChange = ServerTraceStateChange;
