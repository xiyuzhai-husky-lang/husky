use super::*;
use std::panic::{RefUnwindSafe, UnwindSafe};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct __ModelLinkage(pub &'static dyn ModelDyn);

impl std::ops::Deref for __ModelLinkage {
    type Target = dyn ModelDyn;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl PartialEq for __ModelLinkage {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const dyn ModelDyn as *const c_void as usize
            == other.0 as *const dyn ModelDyn as *const c_void as usize
    }
}

impl Eq for __ModelLinkage {}

pub trait ModelDyn: std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
    fn train_dyn<'eval>(
        &self,
        opds: Vec<(Vec<__Register<'eval>>, __Register<'eval>)>,
    ) -> __VMResult<__Register<'eval>>;
    fn eval_dyn<'eval>(
        &self,
        internal: &__Register<'eval>,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>>;
}

pub trait Model:
    std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + Sized + 'static
{
    type Internal: for<'eval> __Registrable<'eval>;
    fn internal_ty_vtable() -> &'static __RegisterTyVTable;

    fn train<'eval>(
        &self,
        opds: Vec<(Vec<__Register<'eval>>, __Register<'eval>)>,
    ) -> __VMResult<Self::Internal>;
    fn eval<'eval>(
        &self,
        internal: &Self::Internal,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>>;
}

impl<T: Model> ModelDyn for T {
    fn train_dyn<'eval>(
        &self,
        opds: Vec<(Vec<__Register<'eval>>, __Register<'eval>)>,
    ) -> __VMResult<__Register<'eval>> {
        Ok(self.train(opds)?.to_register())
    }

    fn eval_dyn<'eval>(
        &self,
        internal: &__Register<'eval>,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>> {
        let internal: &T::Internal = internal.downcast_temp_ref(T::internal_ty_vtable());
        self.eval(internal, arguments)
    }
}
