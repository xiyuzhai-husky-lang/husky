use super::*;
use husky_declarative_ty::*;

impl EtherealTerm {
    pub fn ty_unchecked(
        self,
        db: &dyn EtherealTermDb,
    ) -> TermResult<Either<EtherealTerm, PreludeTypePath>> {
        Ok(match self.raw_ty(db)? {
            Left(raw_ty) => Left(EtherealTerm::from_raw_unchecked(
                db,
                raw_ty,
                TermTypeExpectation::FinalDestinationEqsSort,
            )?),
            Right(prelude_ty_path) => Right(prelude_ty_path),
        })
    }

    pub fn raw_ty(
        self,
        db: &dyn EtherealTermDb,
    ) -> TermResult<Either<DeclarativeTerm, PreludeTypePath>> {
        Ok(match self {
            EtherealTerm::Literal(literal) => Right(literal.ty()),
            // term.raw_ty(db),
            EtherealTerm::Symbol(symbol) => todo!(),
            EtherealTerm::Placeholder(_) => todo!(),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(path) => Left(trai_path_raw_ty(db, path)?),
                TermEntityPath::TypeOntology(path) => Left(ty_ontology_path_raw_ty(db, path)?),
                TermEntityPath::TypeConstructor(path) => {
                    Left(ty_constructor_path_raw_ty(db, path)?)
                }
            },
            EtherealTerm::Category(cat) => Left(cat.ty()?.into()),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => Left(term.raw_ty(db)?),
            EtherealTerm::Subentity(_) => todo!(),
            EtherealTerm::AsTraitSubentity(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        })
    }
}
