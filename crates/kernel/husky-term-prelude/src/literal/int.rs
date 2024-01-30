use super::*;

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermI64Literal {
    pub value: i64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermI128Literal {
    pub value: i128,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermI256Literal {
    pub value: [i128; 2],
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermISizeLiteral {
    pub value: i64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermU64Literal {
    pub value: u64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermU128Literal {
    pub value: u128,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermU256Literal {
    pub value: [u128; 2],
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermUSizeLiteral {
    pub value: u64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermR64Literal {
    pub value: u64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermR128Literal {
    pub value: u128,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermR256Literal {
    pub value: [u128; 2],
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermRSizeLiteral {
    pub value: u64,
}

impl TermLiteral {
    pub fn from_unspecified_int(
        int_ty_path: PreludeIntTypePath,
        val: i128,
        db: &::salsa::Db,
    ) -> TermLiteral {
        match int_ty_path {
            // signed integer types
            PreludeIntTypePath::I8 => TermLiteral::I8(val.try_into().expect("todo")),
            PreludeIntTypePath::I16 => TermLiteral::I16(val.try_into().expect("todo")),
            PreludeIntTypePath::I32 => TermLiteral::I32(val.try_into().expect("todo")),
            PreludeIntTypePath::I64 => {
                TermLiteral::I64(TermI64Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::I128 => {
                TermLiteral::I128(TermI128Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::ISize => {
                TermLiteral::ISize(TermISizeLiteral::new(db, val.try_into().expect("ok")))
            }
            // Unsigned integer types
            PreludeIntTypePath::U8 => {
                TermLiteral::U8(val.try_into().expect("value out of range for u8"))
            }
            PreludeIntTypePath::U16 => {
                TermLiteral::U16(val.try_into().expect("value out of range for u16"))
            }
            PreludeIntTypePath::U32 => {
                TermLiteral::U32(val.try_into().expect("value out of range for u32"))
            }
            PreludeIntTypePath::U64 => {
                TermLiteral::U64(TermU64Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::U128 => {
                TermLiteral::U128(TermU128Literal::new(db, val.try_into().expect("todo")))
            }

            PreludeIntTypePath::USize => {
                TermLiteral::USize(TermUSizeLiteral::new(db, val.try_into().expect("ok")))
            }
            // raw bits
            PreludeIntTypePath::R8 => {
                TermLiteral::R8(val.try_into().expect("value out of range for r8"))
            }
            PreludeIntTypePath::R16 => {
                TermLiteral::R16(val.try_into().expect("value out of range for r16"))
            }
            PreludeIntTypePath::R32 => {
                TermLiteral::R32(val.try_into().expect("value out of range for r32"))
            }
            PreludeIntTypePath::R64 => {
                TermLiteral::R64(TermR64Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::R128 => {
                TermLiteral::R128(TermR128Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::RSize => {
                TermLiteral::RSize(TermRSizeLiteral::new(db, val.try_into().expect("ok")))
            }
        }
    }
}
