use crate::*;

// upcast primitives
impl RegularValue {
    pub fn upcast_i8(val: i8) -> Self {
        RegularValue::I8(val)
    }

    pub fn upcast_i16(val: i16) -> Self {
        RegularValue::I16(val)
    }

    pub fn upcast_i32(val: i32) -> Self {
        RegularValue::I32(val)
    }

    pub fn upcast_i64(val: i64) -> Self {
        RegularValue::I64(val)
    }

    pub fn upcast_i128(val: i128) -> Self {
        RegularValue::I128(val)
    }

    pub fn upcast_isize(val: isize) -> Self {
        RegularValue::ISize(val)
    }

    pub fn upcast_u8(val: u8) -> Self {
        RegularValue::U8(val)
    }

    pub fn upcast_u16(val: u16) -> Self {
        RegularValue::U16(val)
    }

    pub fn upcast_u32(val: u32) -> Self {
        RegularValue::U32(val)
    }

    pub fn upcast_u64(val: u64) -> Self {
        RegularValue::U64(val)
    }

    pub fn upcast_u128(val: u128) -> Self {
        RegularValue::U128(val)
    }

    pub fn upcast_usize(val: usize) -> Self {
        RegularValue::USize(val)
    }

    pub fn upcast_r8(val: u8) -> Self {
        RegularValue::R8(val)
    }

    pub fn upcast_r16(val: u16) -> Self {
        RegularValue::R16(val)
    }

    pub fn upcast_r32(val: u32) -> Self {
        RegularValue::R32(val)
    }

    pub fn upcast_r64(val: u64) -> Self {
        RegularValue::R64(val)
    }

    // f128 is a hypothetical placeholder for a 128-bit floating-point type.
    pub fn upcast_r128(val: u128) -> Self {
        RegularValue::R128(val)
    }

    // fsize is a hypothetical placeholder for an arbitrary-size floating-point type.
    pub fn upcast_rsize(val: usize) -> Self {
        RegularValue::RSize(val)
    }

    pub fn upcast_unit(val: ()) -> Self {
        RegularValue::Unit(())
    }

    pub fn upcast_bool(val: bool) -> Self {
        RegularValue::Bool(val)
    }

    pub fn upcast_char(val: char) -> Self {
        RegularValue::Char(val)
    }
}

// upcast any
impl RegularValue {
    fn upcast_intrinsic_trivial<T>(t: T) -> Self
    where
        T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
    {
        RegularValue::Intrinsic(Box::new(__RegularStandTrivialImpl::upcast(t)))
    }
}
