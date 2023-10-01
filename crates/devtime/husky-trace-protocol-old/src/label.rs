use crate::SampleId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Label(pub usize);

pub struct LabeledData {
    pub sample_id: SampleId,
    pub label: Label,
}
