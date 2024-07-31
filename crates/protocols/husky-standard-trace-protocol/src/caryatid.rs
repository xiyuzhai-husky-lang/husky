#[cfg(feature = "egui")]
mod egui;

use super::*;
use husky_trace_protocol::caryatid::{IsCaryatid, IsCaryatidUiBuffer};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StandardCaryatid {
    // pedestal: StandardPedestal,
}

impl IsCaryatid for StandardCaryatid {
    type Pedestal = StandardPedestal;

    fn pedestal(&self, var_deps: &[ItemPathIdInterface]) -> Self::Pedestal {
        // &self.pedestal
        todo!()
    }

    type UiBuffer = StandardCaryatidUiBuffer;

    fn init_ui_buffer(&self) -> Self::UiBuffer {
        todo!()
    }
}

pub struct StandardCaryatidUiBuffer {}

impl IsCaryatidUiBuffer for StandardCaryatidUiBuffer {
    type Caryatid = StandardCaryatid;

    fn update(&mut self, pedestal: &Self::Caryatid) {
        todo!()
    }
}
