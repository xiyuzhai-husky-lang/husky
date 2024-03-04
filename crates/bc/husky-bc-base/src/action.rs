use husky_ndp_task::action::IsNdpAction;

pub enum BcAction {
    Reduce,
}

impl IsNdpAction for BcAction {}
