use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TraceDocViewSettings {
    layout: TraceDocViewLayout,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TraceDocViewLayout {
    Standard,
}

impl Default for TraceDocViewLayout {
    fn default() -> Self {
        TraceDocViewLayout::Standard
    }
}
