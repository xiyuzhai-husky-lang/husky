use crate::*;
use husky_task_interface::{val_control_flow::ValControlFlow, vm_control_flow::VmControlFlow};

type JsonValue = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceStalk {
    Val(Option<ValControlFlow<JsonValue, JsonValue, ()>>),
    Vm(Option<VmControlFlow<JsonValue, JsonValue, ()>>),
}
