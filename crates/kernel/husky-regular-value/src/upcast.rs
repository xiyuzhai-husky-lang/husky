use crate::*;

// upcast primitives
impl __RegularValue {
    pub fn upcast_i8(val: i8) -> Self {
        __RegularValue::I8(val)
    }

    pub fn upcast_i16(val: i16) -> Self {
        __RegularValue::I16(val)
    }

    pub fn upcast_i32(val: i32) -> Self {
        __RegularValue::I32(val)
    }

    pub fn upcast_i64(val: i64) -> Self {
        __RegularValue::I64(val)
    }

    pub fn upcast_i128(val: i128) -> Self {
        __RegularValue::I128(val)
    }

    pub fn upcast_isize(val: isize) -> Self {
        __RegularValue::ISize(val)
    }

    pub fn upcast_u8(val: u8) -> Self {
        __RegularValue::U8(val)
    }

    pub fn upcast_u16(val: u16) -> Self {
        __RegularValue::U16(val)
    }

    pub fn upcast_u32(val: u32) -> Self {
        __RegularValue::U32(val)
    }

    pub fn upcast_u64(val: u64) -> Self {
        __RegularValue::U64(val)
    }

    pub fn upcast_u128(val: u128) -> Self {
        __RegularValue::U128(val)
    }

    pub fn upcast_usize(val: usize) -> Self {
        __RegularValue::USize(val)
    }

    pub fn upcast_r8(val: u8) -> Self {
        __RegularValue::R8(val)
    }

    pub fn upcast_r16(val: u16) -> Self {
        __RegularValue::R16(val)
    }

    pub fn upcast_r32(val: u32) -> Self {
        __RegularValue::R32(val)
    }

    pub fn upcast_r64(val: u64) -> Self {
        __RegularValue::R64(val)
    }

    // f128 is a hypothetical placeholder for a 128-bit floating-point type.
    pub fn upcast_r128(val: u128) -> Self {
        __RegularValue::R128(val)
    }

    // fsize is a hypothetical placeholder for an arbitrary-size floating-point type.
    pub fn upcast_rsize(val: usize) -> Self {
        __RegularValue::RSize(val)
    }

    pub fn upcast_unit(val: ()) -> Self {
        __RegularValue::Unit(())
    }

    pub fn upcast_bool(val: bool) -> Self {
        __RegularValue::Bool(val)
    }

    pub fn upcast_char(val: char) -> Self {
        __RegularValue::Char(val)
    }
}

// upcast any
impl __RegularValue {
    fn upcast_intrinsic_trivial<T>(t: T) -> Self
    where
        T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
    {
        __RegularValue::Intrinsic(Box::new(__RegularValueStandTrivialImpl::upcast(t)))
    }
}
