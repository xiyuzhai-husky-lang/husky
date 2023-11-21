use husky_regular_value::RegularValue;
use husky_task::link::IsLinkageImpl;

#[derive(Debug, Clone, Copy)]
pub struct TrivialLinkage {}

impl IsLinkageImpl for TrivialLinkage {
    type Value = RegularValue;

    fn eval_fn() -> Self::Value {
        todo!()
    }

    fn eval_gn() -> Self::Value {
        todo!()
    }
}
