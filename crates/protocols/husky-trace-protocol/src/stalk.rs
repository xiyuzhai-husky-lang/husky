use husky_value_interface::{
    ki_control_flow::ValuePresentationKiControlFlow,
    vm_control_flow::ValuePresentationVmControlFlow,
};

use crate::*;

pub type JsonValue = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceStalk {
    None,
    Ki(ValuePresentationKiControlFlow),
    Vm(ValuePresentationVmControlFlow),
}
