use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FloatLiteral {
    Unspecified,
}

impl Neg for FloatLiteral {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
