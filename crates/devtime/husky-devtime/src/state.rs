mod old;

pub use old::*;

use crate::*;

pub type DevtimeState = TraceWorld<TraceNode>;

pub type DevtimeOldState = ServerTraceOldState<TraceNode>;

pub type DevtimeStateChange = ServerTraceStateChange;
