#[cfg(feature = "egui")]
mod egui;

use super::*;
use husky_standard_devsoul_interface::static_var::StandardStaticVarId;
use husky_trace_protocol::caryatid::{IsCaryatid, IsCaryatidUiBuffer};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StandardCaryatid {
    anchors: OrderedSmallVecPairMap<ItemPathIdInterface, Option<StandardStaticVarId>, 2>,
}

impl IsCaryatid for StandardCaryatid {
    type Pedestal = StandardPedestal;

    fn pedestal(&self, var_deps: &[ItemPathIdInterface]) -> Self::Pedestal {
        var_deps
            .iter()
            .copied()
            .filter_map(|dep| {
                self.anchors
                    .get_value(dep)
                    .copied()
                    .flatten()
                    .map(|id| (dep, id))
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
}

impl StandardCaryatid {
    /// empty returns true, intentionally
    pub fn is_specific(&self) -> bool {
        self.anchors.iter().copied().all(|(_, id)| id.is_some())
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

    fn update(&mut self, pedestal: &Self::Caryatid) {
        todo!()
    }
}
