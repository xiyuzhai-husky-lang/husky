use husky_ddp_task::action::IsDdpAction;

use crate::rule::BcRuleId;

pub enum BcAction {
    Reduce,
}

impl IsDdpAction for BcAction {}
