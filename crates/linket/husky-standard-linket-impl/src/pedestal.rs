use super::*;
use husky_item_path_interface::ItemPathIdInterface;
use husky_linket_impl::pedestal::IsPedestal;
use husky_linket_impl::static_var::IsStaticVar;
use static_var::StandardStaticVarId;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StandardPedestal {
    static_var_ids: OrderedSmallVecPairMap<ItemPathIdInterface, StandardStaticVarId, 2>,
}

impl FromIterator<(ItemPathIdInterface, StandardStaticVarId)> for StandardPedestal {
    fn from_iter<T: IntoIterator<Item = (ItemPathIdInterface, StandardStaticVarId)>>(
        iter: T,
    ) -> Self {
        Self {
            static_var_ids: iter.into_iter().collect(),
        }
    }
}

impl IsPedestal for StandardPedestal {
    type StaticVarId = StandardStaticVarId;

    fn exclude<V: IsStaticVar<StandardStaticVarId>>(mut self) -> Self {
        let _ = self.static_var_ids.remove(V::item_path_id_interface());
        self
    }

    fn is_closed(&self, var_deps: &[ItemPathIdInterface]) -> bool {
        var_deps
            .iter()
            .copied()
            .all(|dep| self.static_var_ids.has(dep))
    }
}

impl StandardPedestal {}

#[macro_export]
macro_rules! pedestal {
    ($($static_var: path),*) => {{
        [$((<$static_var>::item_path_id_interface(), <$static_var>::get_id())),*].into_iter().collect()
    }};
}
