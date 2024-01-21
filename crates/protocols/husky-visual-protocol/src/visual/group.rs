use crate::visualize::Visualize;

use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "VisualSerdeId", into = "VisualSerdeId")]
pub struct GroupVisual(VisualId);

impl_visual_serde_id_from_to_for_sub_visual_id! { GroupVisual }

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupVisualData {
    elements: Vec<Visual>,
}

impl GroupVisual {
    pub fn new(elements: Vec<Visual>, visual_synchrotron: &mut VisualSynchrotron) -> Self {
        GroupVisual(visual_synchrotron.alloc_visual(GroupVisualData { elements }))
    }

    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a GroupVisualData {
        let VisualData::Group(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }

    pub fn elements<'a>(&self, visual_synchrotron: &'a VisualSynchrotron) -> &'a [Visual] {
        &self.data(visual_synchrotron).elements
    }
}

impl Visual {
    pub fn new_group_visual(
        elements: Vec<Visual>,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        GroupVisual::new(elements, visual_synchrotron).into()
    }
}
