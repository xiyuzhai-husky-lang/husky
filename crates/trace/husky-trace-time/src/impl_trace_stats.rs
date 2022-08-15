use crate::*;
use husky_entity_route::EntityRoutePtr;
use husky_text::HuskyText;
use husky_word::RootIdentifier;
use vm::{ControlSnapshot, History, VMControl};

impl HuskyTraceTime {
    pub(crate) fn collect_new_trace_stats(&mut self) -> Vec<(TraceStatsKey, Option<TraceStats>)> {
        todo!()
    }
}
