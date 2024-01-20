use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "VisualId", into = "VisualId")]
pub struct RichTextVisual(VisualId);

impl From<VisualId> for RichTextVisual {
    fn from(id: VisualId) -> Self {
        Self(id)
    }
}

impl Into<VisualId> for RichTextVisual {
    fn into(self) -> VisualId {
        self.0
    }
}

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
