use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceNodeData {
    pub trace: TraceData,
    pub expansion: bool,
    pub shown: bool,
}
