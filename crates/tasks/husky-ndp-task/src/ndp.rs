use crate::*;

use self::{action::IsNdpAction, engine::IsNdpEngine, rule::IsNdpRule, state::IsNdpState};

pub trait IsNdp {
    type Action: IsNdpAction;
    type State: IsNdpState;
    type Rule: IsNdpRule;
    type Engine<'a>: IsNdpEngine<'a>;
}
