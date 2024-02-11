use crate::*;

use self::{action::IsDdpAction, engine::IsDdpEngine, rule::IsDdpRule, state::IsDdpState};

pub trait IsDdp {
    type Action: IsDdpAction;
    type State: IsDdpState;
    type Rule: IsDdpRule;
    type Engine<'a>: IsDdpEngine<'a>;
}
