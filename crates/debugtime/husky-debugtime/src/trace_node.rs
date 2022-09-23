use trackable::TrackClone;

use super::*;

#[derive(Debug)]
pub enum TraceNode {
    Uninitialized,
    Initialized {
        trace: Trace,
        expanded: bool,
        shown: bool,
    },
}

impl TrackClone for TraceNode {
    type Output = TraceNodeData;

    fn track(&self) -> Self::Output {
        todo!()
    }
}

impl TraceNode {
    pub(crate) fn initialized(&self) -> bool {
        match self {
            TraceNode::Uninitialized => false,
            TraceNode::Initialized { .. } => true,
        }
    }

    pub(crate) fn toggle_expansion(&mut self) {
        match self {
            TraceNode::Uninitialized => (),
            TraceNode::Initialized {
                trace,
                expanded,
                shown,
            } => *expanded = !*expanded,
        }
    }

    pub(crate) fn expanded(&self) -> bool {
        match self {
            TraceNode::Uninitialized => unreachable!(),
            TraceNode::Initialized { expanded, .. } => *expanded,
        }
    }

    pub(crate) fn toggle_shown(&mut self) {
        match self {
            TraceNode::Uninitialized => (),
            TraceNode::Initialized { shown, .. } => *shown = !*shown,
        }
    }

    pub(crate) fn shown(&self) -> bool {
        match self {
            TraceNode::Uninitialized => unreachable!(),
            TraceNode::Initialized { shown, .. } => *shown,
        }
    }

    pub(crate) fn trace(&self) -> &Trace {
        match self {
            TraceNode::Uninitialized => unreachable!(),
            TraceNode::Initialized {
                trace,
                expanded,
                shown,
            } => trace,
        }
    }

    pub(crate) fn to_data(&self) -> husky_trace_protocol::TraceNodeData {
        match self {
            TraceNode::Uninitialized => unreachable!(),
            TraceNode::Initialized {
                trace,
                expanded,
                shown,
            } => TraceNodeData {
                trace_data: trace.raw_data.clone(),
                expanded: *expanded,
                shown: *shown,
            },
        }
    }
}
