use std::time::Instant;

use crate::*;
use husky_text::HuskyText;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub fn attention(&self) -> &Attention {
        &self.attention
    }

    pub fn set_attention(&mut self, attention: Attention) -> Vec<(TraceStalkKey, TraceStalkData)> {
        self.attention = attention;
        if let Some(sample_id0) = self.attention.opt_sample_id() {
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
                        // self.set_attention_raw(Attention::Specific { sample_id })
                    }
                    EvalError::Normal { .. } => (),
                },
            }
            self.collect_new_trace_stalks()
        } else {
            vec![]
        }
    }

    fn set_attention_raw(&mut self, attention: Attention) {
        self.attention = attention;
    }
}
