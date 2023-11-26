use crate::*;

impl EtherealTerm {
    /// whether two types are trivially convertible
    pub fn is_ty_trivially_convertible_from(
        self,
        db: &::salsa::Db,
        other_ty: Either<Self, PreludeTypePath>,
    ) -> EtherealTermResult<bool> {
        match other_ty {
            Left(other_ty) if other_ty == self => Ok(true),
            Left(other_ty) => {
                // ad hoc
                Ok(false)
            }
            Right(other_ty) => match self {
                EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
                    Ok(ty_path.prelude_ty_path(db) == Some(other_ty))
                }
                _ => {
                    p!(self.debug(db), other_ty.debug(db));
                    todo!()
                }
            },
        }
    }
}
