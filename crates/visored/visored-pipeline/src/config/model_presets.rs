use super::*;
use all_llms::model::AllLlmModel;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VdPipelineModelPreset {
    pub name: String,
    pub model: AllLlmModel,
}

pub type VdPipelineModelPresets = Vec<VdPipelineModelPreset>;
