use husky_ndp_task::ndp::IsNdp;

use crate::{action::BcAction, engine::BcEngine, rule::BcRule, state::BcState};

/// Border Collie Deterministic Decision Process
pub struct BcNdp {}

impl IsNdp for BcNdp {
    type Action = BcAction;

    type State = BcState;

    type Rule = BcRule;

    type Engine<'a> = BcEngine<'a>;
}
