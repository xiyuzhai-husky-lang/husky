use crate::*;
use std::{
    ffi::c_void,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Arc,
};

pub trait __Regular:
    std::any::Any + std::fmt::Debug + RefUnwindSafe + UnwindSafe + __StaticInfo
{
}

pub trait __RegularDyn: std::any::Any + std::fmt::Debug + RefUnwindSafe + UnwindSafe {}

impl PartialEq for Box<dyn __RegularDyn> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
        // self.0 == other.0 && self.1 == other.1
    }
}

impl Clone for Box<dyn __RegularDyn> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Eq for Box<dyn __RegularDyn> {}

impl PartialEq for &'static dyn __RegularDyn {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for &'static dyn __RegularDyn {}

#[derive(Debug, Eq)]
pub struct __BoxDynRegularDyn(Box<dyn __RegularDyn>);

impl PartialEq for __BoxDynRegularDyn {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Clone for __BoxDynRegularDyn {
    fn clone(&self) -> Self {
        todo!()
    }
}
