use crate::*;
use husky_task_interface::{val_control_flow::ValControlFlow, vm_control_flow::VmControlFlow};

pub type JsonValue = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceStalk {
    None,
    Val(ValControlFlow<JsonValue, JsonValue, JsonValue>),
    Vm(VmControlFlow<JsonValue, JsonValue, JsonValue>),
}
