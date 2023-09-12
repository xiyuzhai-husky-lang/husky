use super::*;
use ordered_float::NotNan;
use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FloatLiteral {
    Unspecified(UnspecifiedFloatLiteral),
    F32(NotNan<f32>),
    F64(NotNan<f64>),
}

#[salsa::tracked(db = TokenDataDb, jar = TokenDataJar)]
pub struct UnspecifiedFloatLiteral {
    #[return_ref]
    pub data: String,
}

impl Neg for FloatLiteral {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
