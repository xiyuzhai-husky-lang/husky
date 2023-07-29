use crate::*;
use std::sync::Arc;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum __RegularValueSnapshot {
    Intrinsic(__BoxDynRegularDyn),
    Box(__BoxDynRegularDyn),
    Leash(&'static dyn __RegularDyn),
    SizedRef(Arc<dyn __RegularDyn>),
    SizedRefMut(Box<dyn __RegularDyn>),
    OptionBox(Option<__BoxDynRegularDyn>),
    OptionLeash(Option<&'static dyn __RegularDyn>),
    OptionSizedRef(Option<Arc<dyn __RegularDyn>>),
    OptionSizedRefMut(Option<Box<dyn __RegularDyn>>),
    Unit(()),
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISize(isize),
    U32(u32),
    U64(u64),
    USize(usize),
    R32(u32),
    R64(u64),
    RSize(usize),
    Moved,
}
