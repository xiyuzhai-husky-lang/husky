use crate::*;
use husky_task_interface::{
    val_control_flow::{ValControlFlow, ValuePresentationValControlFlow},
    vm_control_flow::{ValuePresentationVmControlFlow, VmControlFlow},
};

pub type JsonValue = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceStalk {
    None,
    Val(ValuePresentationValControlFlow),
    Vm(ValuePresentationVmControlFlow),
}
