use super::*;

pub struct ImageVisual(VisualId);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageVisualData {}

impl ImageVisual {
    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a ImageVisualData {
        let VisualData::Image(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }
}
