use super::*;

#[derive(Debug)]
pub struct TraceNode {
    pub(crate) trace: Trace,
    pub(crate) expansion: bool,
    pub(crate) shown: bool,
    pub(crate) pin: bool,
    pub(crate) arrival: bool,
    pub(crate) enter: bool,
}

impl TraceNode {
    pub fn to_data(&self) -> husky_trace_protocol::TraceNodeData {
        TraceNodeData {
            trace_data: self.trace.raw_data.clone(),
            expanded: self.expansion,
            shown: self.shown,
            pin: self.pin,
            arrival: self.arrival,
            enter: self.enter,
        }
    }
}
