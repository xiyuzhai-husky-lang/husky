use super::*;
use smallvec::SmallVec;

pub use husky_standard_value::{value_conversion, FromValue, IntoValue};

pub type FnArguments = SmallVec<[Value; 4]>;
// ad hoc
pub type GnArguments = SmallVec<[Value; 4]>;
pub type Value = husky_standard_value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl {
    RitchieFn {
        fn_wrapper: fn(FnArguments) -> Value,
        fn_pointer: fn(),
    },
}

impl IsLinkageImpl for LinkageImpl {
    type Value = Value;
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
