use crate::{Namespace, Ty};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermLiteral {
    data: Literal,
    ty: Ty,
}

impl TermLiteral {
    pub fn ty(&self) -> Ty {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Literal {
    I32(i32),
}
