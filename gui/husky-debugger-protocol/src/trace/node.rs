use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceNodeData {
    pub trace: TraceProps,
    pub expansion: bool,
    pub shown: bool,
}
