use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct VideoVisual(VisualId);

impl_visual_serde_id_from_to_for_sub_visual_id! { VideoVisual }

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
