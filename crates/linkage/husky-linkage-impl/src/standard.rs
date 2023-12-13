use super::*;
use husky_regular_value::RegularValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl {}

impl IsLinkageImpl for LinkageImpl {
    type Value = RegularValue;

    fn eval_fn() -> Self::Value {
        todo!()
    }

    fn eval_gn() -> Self::Value {
        todo!()
    }
}

pub struct LinkageImplSource<T>(pub T);

all_ritchies! {impl_into_linkage_impl}
