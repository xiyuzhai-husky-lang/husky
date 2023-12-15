use super::*;
use husky_regular_value::RegularValue;
use smallvec::SmallVec;

pub type __Arguments = SmallVec<[RegularValue; 4]>;
pub type __Value = RegularValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl {
    RitchieFn {
        fn_wrapper: fn(__Arguments) -> __Value,
        fn_pointer: fn(),
    },
}

impl IsLinkageImpl for LinkageImpl {
    type Value = RegularValue;

    fn eval_fn(self, arguments: SmallVec<[RegularValue; 4]>) -> Self::Value {
        match self {
            LinkageImpl::RitchieFn { fn_wrapper, .. } => fn_wrapper(arguments),
        }
    }

    fn eval_gn(self) -> Self::Value {
        todo!()
    }
}

pub struct LinkageImplSource<T> {
    pub linkage_impl_src: T,
    pub fn_wrapper: fn(SmallVec<[RegularValue; 4]>) -> RegularValue,
    pub gn_wrapper: fn(),
}

all_ritchies! {impl_into_linkage_impl}
