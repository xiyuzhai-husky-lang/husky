use super::*;
use std::panic::{RefUnwindSafe, UnwindSafe};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct __ModelLinkageGroup(pub &'static dyn ModelDyn);

impl std::ops::Deref for __ModelLinkageGroup {
    type Target = dyn ModelDyn;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl PartialEq for __ModelLinkageGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const dyn ModelDyn as *const c_void as usize
            == other.0 as *const dyn ModelDyn as *const c_void as usize
    }
}

impl Eq for __ModelLinkageGroup {}

/// generic argument means something totally different from that in other languages,
/// `generic argument` in other languages translates to `template argument`
/// and `generic argument` in Husky means argument generic over a dataset
///
/// I'm working on a better explanation
pub enum GenericArgument {
    Constant { value: RegularValue },
    NonConstant { values: Vec<RegularValue> },
}

impl GenericArgument {
    pub fn values(&self) -> &[RegularValue] {
        match self {
            GenericArgument::Constant { .. } => panic!(),
            GenericArgument::NonConstant { ref values } => values,
        }
    }

    pub fn value(&self) -> &RegularValue {
        match self {
            GenericArgument::Constant { ref value } => value,
            GenericArgument::NonConstant { .. } => panic!(),
        }
    }
}

pub trait ModelDyn: std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
    fn train_dyn(
        &self,
        arguments: Vec<GenericArgument>,
        labels: Vec<i32>,
    ) -> VMResult<RegularValue>;

    fn eval_dyn(
        &self,
        internal: &RegularValue,
        arguments: &[RegularValue],
    ) -> VMResult<RegularValue>;
}

pub trait Model:
    std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + Sized + 'static
{
    type Internal: __Regular;

    fn train(&self, arguments: Vec<GenericArgument>, labels: Vec<i32>) -> VMResult<Self::Internal>;
    fn eval(&self, internal: &Self::Internal, arguments: &[RegularValue])
        -> VMResult<RegularValue>;
}

impl<T: Model> ModelDyn for T {
    fn train_dyn(
        &self,
        arguments: Vec<GenericArgument>,
        labels: Vec<i32>,
    ) -> VMResult<RegularValue> {
        todo!()
        // Ok(self.train(arguments, labels)?.to_register())
    }

    fn eval_dyn(
        &self,
        internal: &RegularValue,
        arguments: &[RegularValue],
    ) -> VMResult<RegularValue> {
        todo!()
        // let internal: &T::Internal = internal.downcast_temp_ref(T::internal_ty_vtable());
        // self.eval(internal, arguments)
    }
}
