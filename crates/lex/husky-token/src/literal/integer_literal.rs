use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum IntegerLiteral {
    Unspecified,
    R32(u32),
    R64(u32),
}

impl Neg for IntegerLiteral {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
