use self::expr::BcExprs;
use crate::*;
use husky_ndp_task::rule::IsNdpRule;

pub struct BcRuleData {
    inputs: BcExprs,
}

pub struct BcRule {}

impl IsNdpRule for BcRule {
    type Storage = (); // ad hoc

    fn compose(self, other: Self, storage: &mut Self::Storage) -> Self {
        todo!()
    }
}

pub struct BcRuleId {}

pub struct RuleExprId();
