use super::*;

pub struct RichTextVisual(VisualId);

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
