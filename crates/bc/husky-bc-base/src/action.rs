use husky_ndp_task::action::IsNdpAction;

use crate::rule::BcRuleId;

pub enum BcAction {
    Reduce,
}

impl IsNdpAction for BcAction {}
