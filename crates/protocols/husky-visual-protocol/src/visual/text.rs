use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "VisualId", into = "VisualId")]
pub struct TextVisual(VisualId);

impl From<VisualId> for TextVisual {
    fn from(id: VisualId) -> Self {
        Self(id)
    }
}

impl Into<VisualId> for TextVisual {
    fn into(self) -> VisualId {
        self.0
    }
}

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
