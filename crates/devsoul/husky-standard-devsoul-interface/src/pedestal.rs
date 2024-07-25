#[cfg(feature = "egui")]
mod egui;

use super::*;
use husky_devsoul_interface::item_path::ItemPathIdInterface;
use husky_devsoul_interface::pedestal::{IsPedestal, IsPedestalUiBuffer};
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
        Self::from_ids(iter.into_iter())
    }
}

impl IsPedestal for StandardPedestal {
    type StaticVarId = StandardStaticVarId;
    type UiBuffer = MlPedestalUiBuffer;

    fn from_ids(ids: impl Iterator<Item = (ItemPathIdInterface, Self::StaticVarId)>) -> Self {
        Self {
            static_var_ids: ids.collect(),
        }
    }

    fn init_ui_buffer(&self) -> Self::UiBuffer {
        todo!()
        // let base_input_id = match self {
        //     StandardPedestal::Specific(input_id) => input_id,
        //     StandardPedestal::Generic => DeprecatedInputId::from_index(0),
        // };
        // let input_id_to_be = base_input_id.index().to_string();
        // MlPedestalUiBuffer {
        //     base_input_id: base_input_id,
        //     input_id_to_be,
        //     error: None,
        // }
    }

    fn is_closed(&self) -> bool {
        todo!()
        // match self {
        //     StandardPedestal::Specific(_) => true,
        //     StandardPedestal::Generic => false,
        // }
    }
}

impl StandardPedestal {}

#[macro_export]
macro_rules! pedestal {
    ($($static_var: path),*) => {{
        [$((<$static_var>::item_path_id_interface(), <$static_var>::get_id())),*].into_iter().collect()
    }};
}

pub struct MlPedestalUiBuffer {
    input_id_to_be: String,
    error: Option<String>,
}

impl IsPedestalUiBuffer for MlPedestalUiBuffer {
    type Pedestal = StandardPedestal;

    fn update(&mut self, pedestal: &Self::Pedestal) {
        todo!()
        // self.error = None;
        // match pedestal {
        //     StandardPedestal::Specific(input_id) => self.base_input_id = input_id,
        //     StandardPedestal::Generic => (),
        // }
        // *self = pedestal.init_ui_buffer()
    }
}
