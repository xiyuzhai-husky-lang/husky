use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceNodeData {
    pub trace_data: TraceData,
    pub expanded: bool,
    pub shown: bool,
    pub pin: bool,
    pub arrival: bool,
    pub enter: bool,
}
