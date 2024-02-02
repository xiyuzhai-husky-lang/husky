use husky_entity_path::{item_path_menu, TypePath};

use crate::*;

impl DecTerm {
    #[inline(always)]
    pub fn apply(self, db: &::salsa::Db, argument: impl Into<DecTerm>) -> Self {
        ApplicationDecTerm::new(db, self, argument.into()).into()
    }

    pub fn family(self, db: &::salsa::Db) -> DecTermFamily {
        match self {
            DecTerm::EntityPath(ItemPathDecTerm::Type(path)) => DecTermFamily::TypePath(path),
            DecTerm::Category(_) => DecTermFamily::Sort,
            DecTerm::Application(term) => term.function(db).family(db),
            DecTerm::ApplicationOrRitchieCall(term) => term.function(db).family(db),
            DecTerm::LeashOrBitNot(toolchain) => {
                DecTermFamily::TypePath(item_path_menu(db, toolchain).leash_ty_path())
            }
            _ => DecTermFamily::Other,
        }
    }

    /// see `self` as the type of another term, return the type expectation for that term
    pub fn ty_final_destination_expectation(
        self,
        db: &::salsa::Db,
    ) -> DecTermResult<TermTypeExpectation> {
        match self {
            DecTerm::EntityPath(ItemPathDecTerm::Type(path)) => Ok(
                TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path),
            ),
            DecTerm::Category(_) => Ok(TermTypeExpectation::FinalDestinationEqsSort),
            DecTerm::Curry(slf) => slf.return_ty(db).ty_final_destination_expectation(db),
            DecTerm::Application(slf) => slf.function(db).ty_final_destination_expectation(db),
            _ => Ok(TermTypeExpectation::Any),
        }
    }

    pub const PROP: DecTerm = DecTerm::Category(CategoryTerm::new(UniverseTerm::new(0)));

    pub const TYPE: DecTerm = DecTerm::Category(CategoryTerm::new(UniverseTerm::new(1)));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DecTermFamily {
    Sort,
    TypePath(TypePath),
    Other,
}

impl SymbolDecTerm {
    pub(crate) fn ty_family(self, db: &::salsa::Db) -> DecTermFamily {
        self.ty(db)
            .ok()
            .map(|ty| ty.family(db))
            .unwrap_or(DecTermFamily::Other)
    }
}

impl RuneDecTerm {
    pub(crate) fn ty_family(self, db: &::salsa::Db) -> DecTermFamily {
        self.ty(db)
            .ok()
            .map(|ty| ty.family(db))
            .unwrap_or(DecTermFamily::Other)
    }
}
