use serde::{Deserialize, Serialize};
use trace::{Trace, TraceId, TraceKind};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Focus {
    pub opt_input_id: Option<usize>,
}

impl Default for Focus {
    fn default() -> Self {
        Self { opt_input_id: None }
    }
}

impl Focus {
    pub fn effective_opt_input_id_for_subtraces(&self, trace: &Trace) -> Option<usize> {
        match trace.kind {
            TraceKind::Main(_) => todo!(),
            TraceKind::FeatureStmt(_) => todo!(),
            TraceKind::FeatureBranch(_) => todo!(),
            TraceKind::FeatureExpr(_) => todo!(),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt { .. } => todo!(),
            TraceKind::ImprStmt { .. } => todo!(),
            TraceKind::LoopFrame { .. } => todo!(),
            TraceKind::EagerExpr { .. } => todo!(),
            TraceKind::CallHead { .. } => todo!(),
        }
    }

    pub fn figure_key(&self, trace_id: TraceId) -> String {
        format!("{}:{}", self.key(), trace_id.0)
    }

    fn key(&self) -> String {
        if let Some(input_id) = self.opt_input_id {
            format!("{}", input_id)
        } else {
            "g".into()
        }
    }
}
