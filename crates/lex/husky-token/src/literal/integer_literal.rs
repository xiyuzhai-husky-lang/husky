use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum IntegerLiteral {
    Unspecified,
    I32(i32),
    I64(i64),
    R32(u32),
    R64(u32),
}

impl Neg for IntegerLiteral {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
