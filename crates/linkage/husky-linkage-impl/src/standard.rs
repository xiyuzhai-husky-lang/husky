use super::*;
use husky_regular_value::RegularValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl {
    RitchieFn { fn_pointer: fn() },
}

impl IsLinkageImpl for LinkageImpl {
    type Value = RegularValue;

    fn eval_fn() -> Self::Value {
        todo!()
    }

    fn eval_gn() -> Self::Value {
        todo!()
    }
}

pub struct LinkageImplSource<T> {
    pub linkage_impl_src: T,
    pub fn_wrapper: fn(),
    pub gn_wrapper: fn(),
}

all_ritchies! {impl_into_linkage_impl}
