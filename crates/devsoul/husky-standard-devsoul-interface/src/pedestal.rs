#[cfg(feature = "egui")]
mod egui;

use super::DeprecatedInputId;
use super::*;
use husky_devsoul_interface::pedestal::{IsPedestal, IsPedestalUiBuffer};

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StandardPedestal {
    Specific(DeprecatedInputId),
    Generic,
}

impl Default for StandardPedestal {
    fn default() -> Self {
        StandardPedestal::Specific(DeprecatedInputId::from_index(0))
    }
}

impl IsPedestal for StandardPedestal {
    type UiBuffer = MlPedestalUiBuffer;

    fn init_ui_buffer(self) -> Self::UiBuffer {
        let base_input_id = match self {
            StandardPedestal::Specific(input_id) => input_id,
            StandardPedestal::Generic => DeprecatedInputId::from_index(0),
        };
        let input_id_to_be = base_input_id.index().to_string();
        MlPedestalUiBuffer {
            base_input_id: base_input_id,
            input_id_to_be,
            error: None,
        }
    }

    fn is_closed(self) -> bool {
        match self {
            StandardPedestal::Specific(_) => true,
            StandardPedestal::Generic => false,
        }
    }
}

impl StandardPedestal {
    pub fn input_id(self) -> Option<DeprecatedInputId> {
        match self {
            StandardPedestal::Specific(input_id) => Some(input_id),
            StandardPedestal::Generic => None,
        }
    }
}

pub struct MlPedestalUiBuffer {
    base_input_id: DeprecatedInputId,
    input_id_to_be: String,
    error: Option<String>,
}

impl IsPedestalUiBuffer for MlPedestalUiBuffer {
    type Pedestal = StandardPedestal;

    fn update(&mut self, pedestal: Self::Pedestal) {
        self.error = None;
        match pedestal {
            StandardPedestal::Specific(input_id) => self.base_input_id = input_id,
            StandardPedestal::Generic => (),
        }
        *self = pedestal.init_ui_buffer()
    }
}
