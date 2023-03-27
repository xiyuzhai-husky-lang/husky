use husky_entity_path::TypePath;

use crate::*;

impl RawTerm {
    #[inline(always)]
    pub fn apply(self, db: &dyn RawTermDb, argument: impl Into<RawTerm>) -> Self {
        RawTermExplicitApplication::new(db, self, argument.into()).into()
    }

    pub fn family(self, db: &dyn RawTermDb) -> RawTermFamily {
        match self {
            RawTerm::Literal(_) => todo!(),
            RawTerm::Symbol(_) => todo!(),
            RawTerm::Variable(_) => todo!(),
            RawTerm::EntityPath(path) => match path {
                RawTermEntityPath::Form(_) => todo!(),
                RawTermEntityPath::Trait(_) => todo!(),
                RawTermEntityPath::Type(path) => RawTermFamily::TypePath(path),
            },
            RawTerm::Category(_) => RawTermFamily::Sort,
            RawTerm::Universe(_) => todo!(),
            RawTerm::Curry(_) => todo!(),
            RawTerm::Ritchie(_) => todo!(),
            RawTerm::Abstraction(_) => todo!(),
            RawTerm::ExplicitApplication(_) => todo!(),
            RawTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
            RawTerm::Subentity(_) => todo!(),
            RawTerm::AsTraitSubentity(_) => todo!(),
            RawTerm::TraitConstraint(_) => todo!(),
            RawTerm::LeashOrBitNot(_) => todo!(),
            RawTerm::List(_) => todo!(),
        }
    }

    pub const PROP: RawTerm = RawTerm::Category(TermCategory::new(TermUniverse::new(0)));

    pub const TYPE: RawTerm = RawTerm::Category(TermCategory::new(TermUniverse::new(1)));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawTermFamily {
    Sort,
    TypePath(TypePath),
    Unknown,
}

impl RawTermSymbol {
    pub(crate) fn ty_family(self, db: &dyn RawTermDb) -> RawTermFamily {
        self.ty(db)
            .ok()
            .map(|ty| ty.family(db))
            .unwrap_or(RawTermFamily::Unknown)
    }
}

impl RawTermVariable {
    pub(crate) fn ty_family(self, db: &dyn RawTermDb) -> RawTermFamily {
        self.ty(db)
            .ok()
            .map(|ty| ty.family(db))
            .unwrap_or(RawTermFamily::Unknown)
    }
}
