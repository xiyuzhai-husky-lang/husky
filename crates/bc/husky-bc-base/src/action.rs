use crate::rule::BcRuleId;

pub enum BcAction {
    ApplyRule { rule_id: BcRuleId },
    Reduce,
}
