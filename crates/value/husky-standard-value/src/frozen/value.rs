use super::*;
use husky_value_protocol::presentation::EnumUnitValuePresenter;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// we use Arc for everything on heap to reduce clone costs
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum FrozenValue {
    /// useful for snapshot caching on stack
    None,
    Uninit,
    Invalid,
    Moved,
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
    F32(f32),
    F64(f64),
    StringLiteral(StringLiteralId),
    EnumUsize {
        index: usize,
        presenter: EnumUnitValuePresenter,
    },
    Box(Arc<dyn FrozenDyn>),
    Leash(&'static dyn FrozenDyn),
    SizedRef(Arc<dyn FrozenDyn>),
    SizedRefMut(Arc<dyn FrozenDyn>),
    OptionBox(Option<Arc<dyn FrozenDyn>>),
    OptionLeash(Option<&'static dyn FrozenDyn>),
    OptionSizedRef(Option<Arc<dyn FrozenDyn>>),
    OptionSizedRefMut(Option<Arc<dyn FrozenDyn>>),
    Intrinsic(Arc<dyn FrozenDyn>),
}
