use crate::*;

impl Trace {
    pub fn collect_associated_traces(&self, associated_traces: &mut Vec<TraceId>) {
        for line in &self.props.lines {
            for token in &line.tokens {
                if token.value == "]" || token.value == ")" || token.value == "}" {
                    continue;
                }
                if let Some(associated_trace) = &token.opt_associated_trace_id {
                    associated_traces.push(associated_trace.clone())
                }
            }
        }
    }
}
