use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum IntegerLiteral {}

impl Neg for IntegerLiteral {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
