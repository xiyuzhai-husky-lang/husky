use crate::*;

impl<'eval> Trace<'eval> {
    pub fn collect_associated_traces(&self, associated_traces: &mut Vec<Arc<Trace<'eval>>>) {
        for line in &self.lines {
            for token in &line.tokens {
                if let Some(associated_trace) = &token.associated_trace {
                    associated_traces.push(associated_trace.clone())
                }
            }
        }
    }
}
