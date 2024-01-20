use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct TextVisual(VisualId);

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
