use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "VisualSerdeId", into = "VisualSerdeId")]
pub struct TextVisual(VisualId);

impl_visual_serde_id_from_to_for_sub_visual_id! { TextVisual }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextVisualData {}

impl TextVisual {
    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a TextVisualData {
        let VisualData::Text(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }
}
