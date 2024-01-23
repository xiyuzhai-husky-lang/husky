#[cfg(feature = "egui")]
mod egui;

use super::InputId;
use super::*;
use husky_task_interface::pedestal::{IsPedestal, IsPedestalFull, IsPedestalUiBuffer};

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MlPedestal {
    Specific(InputId),
    Generic,
}

impl Default for MlPedestal {
    fn default() -> Self {
        MlPedestal::Specific(InputId::from_index(0))
    }
}

impl IsPedestal for MlPedestal {
    type UiBuffer = MlPedestalUiBuffer;

    fn init_ui_buffer(self) -> Self::UiBuffer {
        let base_input_id = match self {
            MlPedestal::Specific(input_id) => input_id,
            MlPedestal::Generic => InputId::from_index(0),
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
            MlPedestal::Specific(_) => true,
            MlPedestal::Generic => false,
        }
    }
}

impl MlPedestal {
    pub fn input_id(self) -> Option<InputId> {
        match self {
            MlPedestal::Specific(input_id) => Some(input_id),
            MlPedestal::Generic => None,
        }
    }
}

pub struct MlPedestalUiBuffer {
    base_input_id: InputId,
    input_id_to_be: String,
    error: Option<String>,
}

impl IsPedestalUiBuffer for MlPedestalUiBuffer {
    type Pedestal = MlPedestal;

    fn update(&mut self, pedestal: Self::Pedestal) {
        self.error = None;
        match pedestal {
            MlPedestal::Specific(input_id) => self.base_input_id = input_id,
            MlPedestal::Generic => (),
        }
        *self = pedestal.init_ui_buffer()
    }
}
