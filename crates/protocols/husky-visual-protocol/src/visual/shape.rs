use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct ShapeVisual(VisualId);

impl From<VisualId> for ShapeVisual {
    fn from(id: VisualId) -> Self {
        Self(id)
    }
}

impl Into<VisualId> for ShapeVisual {
    fn into(self) -> VisualId {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeVisualData {}

impl ShapeVisual {
    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a ShapeVisualData {
        let VisualData::Shape(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }
}
