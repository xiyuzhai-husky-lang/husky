use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MeshVisual(VisualId);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeshVisualData {}

impl MeshVisual {
    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a MeshVisualData {
        let VisualData::Mesh(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }
}
