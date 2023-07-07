use husky_entity_path::TypePath;

use crate::*;

impl DeclarativeTerm {
    #[inline(always)]
    pub fn apply(self, db: &dyn DeclarativeTermDb, argument: impl Into<DeclarativeTerm>) -> Self {
        DeclarativeTermExplicitApplication::new(db, self, argument.into()).into()
    }

    pub fn family(self, db: &dyn DeclarativeTermDb) -> DeclarativeTermFamily {
        match self {
            DeclarativeTerm::Literal(_) => todo!(),
            DeclarativeTerm::Symbol(_) => DeclarativeTermFamily::Other,
            DeclarativeTerm::Variable(_) => todo!(),
            DeclarativeTerm::EntityPath(path) => match path {
                DeclarativeTermEntityPath::Form(_) => todo!(),
                DeclarativeTermEntityPath::Trait(_) => todo!(),
                DeclarativeTermEntityPath::Type(path) => DeclarativeTermFamily::TypePath(path),
            },
            DeclarativeTerm::Category(_) => DeclarativeTermFamily::Sort,
            DeclarativeTerm::Universe(_) => todo!(),
            DeclarativeTerm::Curry(_) => todo!(),
            DeclarativeTerm::Ritchie(_) => todo!(),
            DeclarativeTerm::Abstraction(_) => todo!(),
            DeclarativeTerm::ExplicitApplication(term) => term.function(db).family(db),
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
            DeclarativeTerm::Subentity(_) => todo!(),
            DeclarativeTerm::AsTraitSubentity(_) => todo!(),
            DeclarativeTerm::TraitConstraint(_) => todo!(),
            DeclarativeTerm::LeashOrBitNot(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
            DeclarativeTerm::Wrapper(_) => todo!(),
        }
    }

    /// see `self` as the type of another term, return the type expectation for that term
    pub fn ty_expectation(
        self,
        db: &dyn DeclarativeTermDb,
    ) -> DeclarativeTermResult<TermTypeExpectation> {
        Ok(match self {
            DeclarativeTerm::EntityPath(DeclarativeTermEntityPath::Type(path)) => {
                TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path)
            }
            DeclarativeTerm::Category(_) => TermTypeExpectation::FinalDestinationEqsSort,
            DeclarativeTerm::Curry(_) => todo!(),
            DeclarativeTerm::ExplicitApplication(_) => todo!(),
            _ => TermTypeExpectation::Any,
        })
    }

    pub const PROP: DeclarativeTerm =
        DeclarativeTerm::Category(TermCategory::new(TermUniverse::new(0)));

    pub const TYPE: DeclarativeTerm =
        DeclarativeTerm::Category(TermCategory::new(TermUniverse::new(1)));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeclarativeTermFamily {
    Sort,
    TypePath(TypePath),
    Other,
}

impl DeclarativeTermSymbol {
    pub(crate) fn ty_family(self, db: &dyn DeclarativeTermDb) -> DeclarativeTermFamily {
        self.ty(db)
            .ok()
            .map(|ty| ty.family(db))
            .unwrap_or(DeclarativeTermFamily::Other)
    }
}

impl DeclarativeTermVariable {
    pub(crate) fn ty_family(self, db: &dyn DeclarativeTermDb) -> DeclarativeTermFamily {
        self.ty(db)
            .ok()
            .map(|ty| ty.family(db))
            .unwrap_or(DeclarativeTermFamily::Other)
    }
}
