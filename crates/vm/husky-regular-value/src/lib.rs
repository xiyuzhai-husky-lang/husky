mod regular;
mod snapshot;
mod static_info;

pub use self::regular::*;
pub use self::snapshot::*;
pub use self::static_info::*;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
#[derive(Debug)]
#[repr(u8)]
pub enum __RegularValue {
    /// `Box<T>`
    Box(Box<dyn __RegularDyn>),
    // ad hoc
    /// `~T`
    Leash(&'static dyn __RegularDyn),
    /// `&T` for T Sized
    SizedRef(*mut dyn __RegularDyn),
    /// `&mut T` for T Sized
    SizedRefMut(*mut dyn __RegularDyn),
    OptionBox(Option<Box<dyn __RegularDyn>>),
    OptionLeash(Option<&'static dyn __RegularDyn>),
    OptionRef(Option<&'static dyn __RegularDyn>),
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
    /// T where T is not in above cases
    Intrinsic(Box<dyn __RegularDyn>),
    Moved,
}
