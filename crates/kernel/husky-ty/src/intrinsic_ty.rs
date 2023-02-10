use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct IntrinsicType(ReducedTerm);

impl IntrinsicType {
    pub fn reduced_term(self) -> ReducedTerm {
        self.0
    }
}

pub(crate) fn intrinsic_ty(db: &dyn TypeDb, ty: ReducedTerm) -> IntrinsicType {
    todo!()
}
