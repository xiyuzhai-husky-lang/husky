use husky_entity_path::{item_path_menu, TypePath};

use crate::*;

impl DeclarativeTerm {
    #[inline(always)]
    pub fn apply(self, db: &::salsa::Db, argument: impl Into<DeclarativeTerm>) -> Self {
        DeclarativeTermExplicitApplication::new(db, self, argument.into()).into()
    }

    pub fn family(self, db: &::salsa::Db) -> DeclarativeTermFamily {
        match self {
            DeclarativeTerm::EntityPath(DeclarativeTermEntityPath::Type(path)) => {
                DeclarativeTermFamily::TypePath(path)
            }
            DeclarativeTerm::Category(_) => DeclarativeTermFamily::Sort,
            DeclarativeTerm::ExplicitApplication(term) => term.function(db).family(db),
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(term) => term.function(db).family(db),
            DeclarativeTerm::LeashOrBitNot(toolchain) => {
                DeclarativeTermFamily::TypePath(item_path_menu(db, toolchain).leash_ty_path())
            }
            _ => DeclarativeTermFamily::Other,
        }
    }

    /// see `self` as the type of another term, return the type expectation for that term
    pub fn ty_final_destination_expectation(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeTermResult<TermTypeExpectation> {
        match self {
            DeclarativeTerm::EntityPath(DeclarativeTermEntityPath::Type(path)) => Ok(
                TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path),
            ),
            DeclarativeTerm::Category(_) => Ok(TermTypeExpectation::FinalDestinationEqsSort),
            DeclarativeTerm::Curry(slf) => slf.return_ty(db).ty_final_destination_expectation(db),
            DeclarativeTerm::ExplicitApplication(slf) => {
                slf.function(db).ty_final_destination_expectation(db)
            }
            _ => Ok(TermTypeExpectation::Any),
        }
    }

    pub const PROP: DeclarativeTerm =
        DeclarativeTerm::Category(TermCategory::new(TermUniverse::new(0)));

    pub const TYPE: DeclarativeTerm =
        DeclarativeTerm::Category(TermCategory::new(TermUniverse::new(1)));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeclarativeTermFamily {
    Sort,
    TypePath(TypePath),
    Other,
}

impl DeclarativeTermSymbol {
    pub(crate) fn ty_family(self, db: &::salsa::Db) -> DeclarativeTermFamily {
        self.ty(db)
            .ok()
            .map(|ty| ty.family(db))
            .unwrap_or(DeclarativeTermFamily::Other)
    }
}

impl DeclarativeTermRune {
    pub(crate) fn ty_family(self, db: &::salsa::Db) -> DeclarativeTermFamily {
        self.ty(db)
            .ok()
            .map(|ty| ty.family(db))
            .unwrap_or(DeclarativeTermFamily::Other)
    }
}
