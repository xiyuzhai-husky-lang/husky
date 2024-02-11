use self::expr::BcExprs;
use crate::*;
use husky_ddp_task::rule::IsDdpRule;

pub struct BcRuleData {
    inputs: BcExprs,
}

pub struct BcRule {}

impl IsDdpRule for BcRule {
    type Storage = (); // ad hoc

    fn compose(self, other: Self, storage: &mut Self::Storage) -> Self {
        todo!()
    }
}

pub struct BcRuleId {}

pub struct RuleExprId();
