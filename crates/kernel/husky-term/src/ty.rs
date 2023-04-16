use super::*;
use husky_raw_ty::*;

impl Term {
    pub fn ty_unchecked(self, db: &dyn TermDb) -> TermResult<Either<Term, PreludeTypePath>> {
        Ok(match self.raw_ty(db)? {
            Left(raw_ty) => Left(Term::from_raw_unchecked(
                db,
                raw_ty,
                TermTypeExpectation::FinalDestinationEqsSort,
            )?),
            Right(prelude_ty_path) => Right(prelude_ty_path),
        })
    }

    pub fn raw_ty(self, db: &dyn TermDb) -> TermResult<Either<RawTerm, PreludeTypePath>> {
        Ok(match self {
            Term::Literal(literal) => Right(literal.ty()),
            // term.raw_ty(db),
            Term::Symbol(symbol) => todo!(),
            Term::Placeholder(_) => todo!(),
            Term::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(path) => Left(trai_path_raw_ty(db, path)?),
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
}
