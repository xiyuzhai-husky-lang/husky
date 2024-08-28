#[cfg(feature = "egui")]
mod egui;

use super::*;
use husky_item_path_interface::ItemPathIdInterface;
use husky_trace_protocol::{
    anchor::Anchor,
    caryatid::{IsCaryatid, IsCaryatidUiBuffer},
};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StandardCaryatid {
    anchors: OrderedSmallVecPairMap<ItemPathIdInterface, Anchor<StandardVarId>, 2>,
}

impl std::ops::Index<ItemPathIdInterface> for StandardCaryatid {
    type Output = Anchor<StandardVarId>;

    fn index(&self, item_path_id_interface: ItemPathIdInterface) -> &Self::Output {
        &self.anchors[item_path_id_interface].1
    }
}

impl IsCaryatid for StandardCaryatid {
    type Pedestal = StandardPedestal;

    fn pedestal(&self, var_deps: &[ItemPathIdInterface]) -> Option<Self::Pedestal> {
        var_deps
            .iter()
            .copied()
            .map(|dep| {
                self.anchors
                    .get_value(dep)
                    .copied()
                    .map(|anchor| match anchor {
                        Anchor::Specific(id) => Some((dep, id)),
                        Anchor::Generic { limit } => None, // ad hoc, maybe fixed point?
                    })
                    .flatten()
            })
            .collect()
    }

    fn covers(&self, var_deps: &[ItemPathIdInterface]) -> bool {
        var_deps.iter().copied().all(|dep| self.anchors.has(dep))
    }

    type UiBuffer = StandardCaryatidUiBuffer;

    fn init_ui_buffer(&self) -> Self::UiBuffer {
        StandardCaryatidUiBuffer {}
    }

    fn with_extra_var_deps(&self, var_deps: &[ItemPathIdInterface]) -> Self {
        let mut slf = self.clone();
        slf.anchors.extend(
            var_deps
                .iter()
                .map(|&var_dep| (var_dep, Anchor::Generic { limit: 100 })), // ad hoc
        );
        slf
    }
}

impl StandardCaryatid {
    /// empty returns true, intentionally
    pub fn is_specific(&self) -> bool {
        self.anchors
            .iter()
            .copied()
            .all(|(_, anchor)| !anchor.is_generic())
    }
}

#[test]
fn standard_caryatid_is_specific_works() {
    let mut caryatid = StandardCaryatid::default();
    assert!(caryatid.is_specific());
}

pub struct StandardCaryatidUiBuffer {}

impl IsCaryatidUiBuffer for StandardCaryatidUiBuffer {
    type Caryatid = StandardCaryatid;

    fn update(&mut self, caryatid: &Self::Caryatid) {
        // ad hoc, todo!()
    }
}
