use self::term::EthTerm;
use crate::*;

impl EthTerm {
    /// whether two types are trivially convertible
    pub fn is_ty_trivially_convertible_from(
        self,
        db: &::salsa::Db,
        other_ty: Either<Self, PreludeTypePath>,
    ) -> EthTermResult<bool> {
        match other_ty {
            Left(other_ty) if other_ty == self => Ok(true),
            Left(_other_ty) => {
                // ad hoc
                Ok(false)
            }
            Right(other_ty) => match self {
                EthTerm::EntityPath(ItemPathTerm::TypeOntology(ty_path)) => {
                    Ok(ty_path.prelude_ty_path(db) == Some(other_ty))
                }
                _ => {
                    use husky_print_utils::p;
                    p!(self.debug(db), other_ty.debug(db));
                    todo!()
                }
            },
        }
    }
}
