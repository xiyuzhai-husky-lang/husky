use ordered_float::NotNan;
use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FloatLiteral {
    Unspecified,
    F32(NotNan<f32>),
    F64(NotNan<f64>),
}

impl Neg for FloatLiteral {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
