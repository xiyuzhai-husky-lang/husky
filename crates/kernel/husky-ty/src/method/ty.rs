use super::*;

pub(super) fn ty_method_ty(
    db: &dyn TypeDb,
    term: Term,
    ident: Ident,
) -> TypeResult<Option<(TypeMethodDisambiguation, TypeResult<Term>)>> {
    todo!()
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
