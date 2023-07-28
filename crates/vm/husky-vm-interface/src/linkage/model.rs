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
    Constant { value: __RegularValue },
    NonConstant { values: Vec<__RegularValue> },
}

impl GenericArgument {
    pub fn values(&self) -> &[__RegularValue] {
        match self {
            GenericArgument::Constant { .. } => panic!(),
            GenericArgument::NonConstant { ref values } => values,
        }
    }

    pub fn value(&self) -> &__RegularValue {
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
    ) -> __VMResult<__RegularValue>;

    fn eval_dyn(
        &self,
        internal: &__RegularValue,
        arguments: &[__RegularValue],
    ) -> __VMResult<__RegularValue>;
}

pub trait Model:
    std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + Sized + 'static
{
    type Internal: __Registrable;
    fn internal_ty_vtable() -> &'static __RegisterTyVTable;

    fn train(
        &self,
        arguments: Vec<GenericArgument>,
        labels: Vec<i32>,
    ) -> __VMResult<Self::Internal>;
    fn eval(
        &self,
        internal: &Self::Internal,
        arguments: &[__RegularValue],
    ) -> __VMResult<__RegularValue>;
}

impl<T: Model> ModelDyn for T {
    fn train_dyn(
        &self,
        arguments: Vec<GenericArgument>,
        labels: Vec<i32>,
    ) -> __VMResult<__RegularValue> {
        Ok(self.train(arguments, labels)?.to_register())
    }

    fn eval_dyn(
        &self,
        internal: &__RegularValue,
        arguments: &[__RegularValue],
    ) -> __VMResult<__RegularValue> {
        let internal: &T::Internal = internal.downcast_temp_ref(T::internal_ty_vtable());
        self.eval(internal, arguments)
    }
}
