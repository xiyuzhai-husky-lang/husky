use husky_ddp_task::ddp::IsDdp;

use crate::{action::BcAction, engine::BcEngine, rule::BcRule, state::BcState};

/// Border Collie Deterministic Decision Process
pub struct BcDdp {}

impl IsDdp for BcDdp {
    type Action = BcAction;

    type State = BcState;

    type Rule = BcRule;

    type Engine<'a> = BcEngine<'a>;
}
