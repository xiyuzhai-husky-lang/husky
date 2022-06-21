use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceNodeData {
    pub raw_data: TraceRawData,
    pub expanded: bool,
    pub shown: bool,
}
