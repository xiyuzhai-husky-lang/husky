use crate::*;
use husky_task_interface::{
    val_control_flow::ValuePresentationValControlFlow,
    vm_control_flow::ValuePresentationVmControlFlow,
};

pub type JsonValue = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceStalk {
    None,
    Ki(ValuePresentationValControlFlow),
    Vm(ValuePresentationVmControlFlow),
}
