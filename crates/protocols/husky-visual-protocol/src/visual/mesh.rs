use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "VisualId", into = "VisualId")]
pub struct MeshVisual(VisualId);

impl From<VisualId> for MeshVisual {
    fn from(id: VisualId) -> Self {
        Self(id)
    }
}

impl Into<VisualId> for MeshVisual {
    fn into(self) -> VisualId {
        self.0
    }
}

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
