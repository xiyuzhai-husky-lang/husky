use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct ShapeVisual(VisualId);

impl_visual_serde_id_from_to_for_sub_visual_id! { ShapeVisual }

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
