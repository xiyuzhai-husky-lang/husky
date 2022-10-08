use crate::Ty;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermLiteral {
    data: TermLiteralData,
    ty: Ty,
}

impl TermLiteral {
    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub fn data(&self) -> &TermLiteralData {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TermLiteralData {
    I32(i32),
    // mom
}
