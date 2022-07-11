use std::time::Instant;

use crate::*;
use husky_text::HuskyText;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub fn restriction(&self) -> &Restriction {
        &self.restriction
    }

    pub fn set_restriction(
        &mut self,
        restriction: Restriction,
    ) -> Vec<(TraceStalkKey, TraceStalkData)> {
        self.restriction = restriction;
        if let Some(sample_id0) = self.restriction.opt_sample_id() {
            let main_file = self.eval_time().compile_time().main_file();
            let main_feature_repr = self.eval_time().main_feature_repr(main_file);
            match self
                .eval_time_singleton
                .husky_feature_eval_repr(&main_feature_repr, sample_id0)
            {
                Ok(_) => (),
                Err(e) => match e {
                    EvalError::FromBatch { sample_id, .. } => {
                        todo!()
                        // self.set_restriction_raw(Restriction::Specific { sample_id })
                    }
                    EvalError::Normal { .. } => (),
                },
            }
            self.collect_new_trace_stalks()
        } else {
            vec![]
        }
    }

    fn set_restriction_raw(&mut self, restriction: Restriction) {
        self.restriction = restriction;
    }
}
