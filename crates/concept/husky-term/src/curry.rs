use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermCurry {
    from: Ty,
    to: Ty,
    ty: Ty,
}

impl TermCurry {
    pub fn ty(&self) -> Ty {
        self.ty
    }
}
