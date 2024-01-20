use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct VideoVisual(VisualId);

impl From<VisualId> for VideoVisual {
    fn from(id: VisualId) -> Self {
        Self(id)
    }
}

impl Into<VisualId> for VideoVisual {
    fn into(self) -> VisualId {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoVisualData {}

impl VideoVisual {
    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a VideoVisualData {
        let VisualData::Video(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }
}
