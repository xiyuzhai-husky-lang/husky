use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceNodeData {
    pub trace: TraceRawData,
    pub expansion: bool,
    pub shown: bool,
}
