#[cfg(feature = "egui")]
mod egui;

use super::*;
use husky_item_path_interface::ItemPathIdInterface;
use husky_trace_protocol::{
    anchor::Anchor,
    caryatid::{IsCaryatid, IsCaryatidUiBuffer},
    windlass::Windlass,
};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StandardCaryatid {
    windlasses: OrderedSmallVecPairMap<ItemPathIdInterface, Windlass<StandardVarId>, 2>,
}

impl std::ops::Index<ItemPathIdInterface> for StandardCaryatid {
    type Output = Windlass<StandardVarId>;

    fn index(&self, item_path_id_interface: ItemPathIdInterface) -> &Self::Output {
        &self.windlasses[item_path_id_interface].1
    }
}

impl IsCaryatid for StandardCaryatid {
    type Pedestal = StandardPedestal;

    fn pedestal(&self, var_deps: &[ItemPathIdInterface]) -> Option<Self::Pedestal> {
        var_deps
            .iter()
            .copied()
            .map(|dep| {
                self.windlasses
                    .get_value(dep)
                    .copied()
                    .map(|windlass| match windlass {
                        Windlass::Specific(var_id)
                        | Windlass::Generic {
                            base: Some(var_id), ..
                        } => Some((dep, var_id)),
                        _ => None,
                    })
                    .flatten()
            })
            .collect()
    }

    fn has(&self, var_deps: &[ItemPathIdInterface]) -> bool {
        todo!()
        // var_deps.iter().copied().all(|dep| self.winlasses.has(dep))
    }

    type UiBuffer = StandardCaryatidUiBuffer;

    fn init_ui_buffer(&self) -> Self::UiBuffer {
        StandardCaryatidUiBuffer {}
    }

    fn with_extra_var_deps(&self, var_deps: &[ItemPathIdInterface]) -> Self {
        let mut slf = self.clone();
        slf.windlasses.extend(
            var_deps.iter().map(|&var_dep| {
                (
                    var_dep,
                    Windlass::Generic {
                        base: None,
                        limit: 100,
                    },
                )
            }), // ad hoc
        );
        slf
    }
}

impl StandardCaryatid {
    /// empty returns true, intentionally
    pub fn is_specific(&self) -> bool {
        self.windlasses
            .iter()
            .copied()
            .all(|(_, windlass)| !windlass.is_generic())
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
