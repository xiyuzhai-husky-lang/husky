mod entity_path;
mod field;
mod method;

pub use self::entity_path::*;
pub use self::field::*;
pub use self::method::*;

use crate::*;
use husky_raw_ty::{ty_constructor_path_raw_ty, ty_ontology_path_raw_ty};

impl Term {
    pub fn ty(self, db: &dyn TermDb, toolchain: Toolchain) -> TermResult<Term> {
        match self.raw_ty(db)? {
            Left(raw_ty) => Term::ty_from_raw(db, raw_ty),
            Right(_) => todo!(),
        }
    }

    pub(crate) fn raw_ty(self, db: &dyn TermDb) -> TermResult<Either<RawTerm, PreludeTypePath>> {
        Ok(match self {
            Term::Literal(literal) => Right(literal.ty()),
            // term.raw_ty(db),
            Term::Symbol(_) => todo!(),
            Term::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(path) => Left(ty_ontology_path_raw_ty(db, path)?),
                TermEntityPath::TypeConstructor(path) => {
                    Left(ty_constructor_path_raw_ty(db, path)?)
                }
            },
            Term::Category(cat) => Left(cat.ty()?.into()),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(term) => Left(term.raw_ty(db)?),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        })
    }

    /// whether two types are trivially convertible
    pub fn is_ty_trivially_convertible_from(
        self,
        db: &dyn TermDb,
        other_ty: Either<Self, PreludeTypePath>,
    ) -> TermResult<bool> {
        match other_ty {
            Left(other_ty) if other_ty == self => Ok(true),
            Left(other_ty) => {
                // ad hoc
                Ok(false)
            }
            Right(other_ty) => match self {
                Term::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
                    Ok(ty_path.prelude_ty_path(db) == Some(other_ty))
                }
                _ => todo!(),
            },
        }
    }
}
