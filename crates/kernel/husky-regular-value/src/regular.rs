mod snapshot;
mod snapshot_dyn;
mod stand;
mod stand_dyn;
mod r#static;
mod static_dyn;

pub use self::r#static::*;
pub use self::snapshot::*;
pub use self::snapshot_dyn::*;
pub use self::stand::*;
pub use self::stand_dyn::*;
pub use self::static_dyn::*;

use crate::*;
use std::{
    ffi::c_void,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Arc,
};

pub trait __Regular: std::fmt::Debug {
    type __Static: __RegularStatic;
}

impl<T> __Regular for &mut T
where
    T: __Regular,
{
    type __Static = __StaticRefMut<T>;
}

impl PartialEq for Box<dyn __RegularStaticDyn> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
        // self.0 == other.0 && self.1 == other.1
    }
}

impl Clone for Box<dyn __RegularStaticDyn> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Eq for Box<dyn __RegularStaticDyn> {}

impl PartialEq for &'static dyn __RegularStaticDyn {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for &'static dyn __RegularStaticDyn {}
