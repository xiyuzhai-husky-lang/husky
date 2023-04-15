use super::*;

impl Term {
    pub(super) fn ty_method_ty(
        self,
        db: &dyn TermDb,
        ident: Ident,
    ) -> TermResult<Option<(TypeMethodDisambiguation, TermResult<Term>)>> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeMethodDisambiguation {
    indirections: SmallVec<[MethodIndirection; 2]>,
    ty_path: TypePath,
}

impl TypeMethodDisambiguation {
    pub fn indirections(&self) -> &[MethodIndirection] {
        &self.indirections
    }

    pub fn ty_path(&self) -> TypePath {
        self.ty_path
    }
}
