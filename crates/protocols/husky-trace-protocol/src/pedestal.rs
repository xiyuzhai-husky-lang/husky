use crate::*;
use husky_devsoul_interface::pedestal::{IsPedestal, IsPedestalFull};

use ui::ui::IsUi;

use self::view::action::TraceViewActionBuffer;

pub trait PedestalUi<Ui: IsUi>: IsPedestalFull {
    fn pedestal_ui<TraceProtocol>(
        &self,
        ui: &mut Ui,
        pedestal_buffer: &mut Self::UiBuffer,
        action_buffer: &mut TraceViewActionBuffer<TraceProtocol>,
    ) where
        TraceProtocol: IsTraceProtocol<Pedestal = Self>;
}

pub type TracePedestalUiBuffer<TraceProtocol> =
    <<TraceProtocol as IsTraceProtocol>::Pedestal as IsPedestal>::UiBuffer;
