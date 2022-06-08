use super::*;
use serde::{Deserialize, Serialize};

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

    pub fn figure_control_key(&self, trace_props: &TraceProps) -> String {
        match trace_props.kind {
            TraceKind::LoopFrame => {
                format!("{}", trace_props.parent.unwrap().0)
            }
            _ => format!("{}", trace_props.id.0),
        }
    }
}
