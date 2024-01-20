use super::*;

pub struct VideoVisual(VisualId);

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
