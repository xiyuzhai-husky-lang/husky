use crate::*;
use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FloatLiteral {}

impl Neg for FloatLiteral {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
