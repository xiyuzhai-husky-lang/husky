use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "VisualSerdeId", into = "VisualSerdeId")]
pub struct MathVisual(VisualId);

impl_visual_serde_id_from_to_for_sub_visual_id! { MathVisual }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathVisualData {}

pub type MathVisuals = Vec<MathVisual>;
