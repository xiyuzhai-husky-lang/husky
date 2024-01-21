use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "VisualSerdeId", into = "VisualSerdeId")]
pub struct RichTextVisual(VisualId);

impl_visual_serde_id_from_to_for_sub_visual_id! { RichTextVisual }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichTextVisualData {}

impl RichTextVisual {
    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a RichTextVisualData {
        let VisualData::RichText(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }
}
