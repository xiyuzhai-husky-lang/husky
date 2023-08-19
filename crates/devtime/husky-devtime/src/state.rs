mod old;

pub use old::*;

use crate::*;

pub type DevtimeState = ServerTraceState<TraceNode>;

pub type DevtimeOldState = ServerTraceOldState<TraceNode>;

pub type DevtimeStateChange = ServerTraceStateChange;
