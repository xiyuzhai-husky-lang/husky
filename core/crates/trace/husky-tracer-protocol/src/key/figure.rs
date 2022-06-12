use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FigureKey {
    Point { this: TraceId },
}

impl FigureKey {
    pub fn new(trace_props: &TraceData) -> FigureKey {
        todo!()
    }
}
