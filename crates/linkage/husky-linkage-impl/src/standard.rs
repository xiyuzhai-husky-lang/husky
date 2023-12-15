use super::*;
use husky_regular_value::RegularValue;
use smallvec::SmallVec;

pub type FnArguments = SmallVec<[RegularValue; 4]>;
// ad hoc
pub type GnArguments = SmallVec<[RegularValue; 4]>;
pub type Value = RegularValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl {
    RitchieFn {
        fn_wrapper: fn(FnArguments) -> Value,
        fn_pointer: fn(),
    },
}

impl IsLinkageImpl for LinkageImpl {
    type Value = RegularValue;
    type FnArguments = FnArguments;
    type GnArguments = GnArguments;

    fn eval_fn(self, arguments: FnArguments) -> Self::Value {
        match self {
            LinkageImpl::RitchieFn { fn_wrapper, .. } => fn_wrapper(arguments),
        }
    }

    fn eval_gn(self) -> Self::Value {
        todo!()
    }
}

pub struct LinkageImplSource<T>(pub T);

all_ritchies! {impl_into_linkage_impl}
