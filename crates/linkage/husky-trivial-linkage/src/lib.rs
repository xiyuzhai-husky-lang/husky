use husky_regular_value::RegularValue;
use husky_task::linkage::IsLinkage;

#[derive(Debug, Clone, Copy)]
pub struct TrivialLinkage {}

impl IsLinkage for TrivialLinkage {
    type Value = RegularValue;

    fn eval_fn() -> Self::Value {
        todo!()
    }

    fn eval_gn() -> Self::Value {
        todo!()
    }
}
