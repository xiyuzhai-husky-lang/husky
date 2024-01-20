use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct GroupVisual(VisualId);

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupVisualData {}

impl GroupVisual {
    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a GroupVisualData {
        let VisualData::Group(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }
}
