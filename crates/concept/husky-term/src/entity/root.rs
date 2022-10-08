use husky_word::RootBuiltinIdentifier;

use crate::*;
use RootBuiltinIdentifier::*;

impl TermEntity {
    pub(crate) fn i32(db: &dyn TermQuery) -> TermPtr {
        Self::root_entity(db, I32)
    }

    pub(crate) fn i64(db: &dyn TermQuery) -> TermPtr {
        Self::root_entity(db, I64)
    }

    pub(crate) fn module(db: &dyn TermQuery) -> TermPtr {
        Self::root_entity(db, ModuleType)
    }

    pub(crate) fn trai(db: &dyn TermQuery) -> TermPtr {
        Self::root_entity(db, TraitType)
    }

    pub(crate) fn std(db: &dyn TermQuery) -> TermPtr {
        Self::root_entity(db, Std)
    }

    fn root_entity(db: &dyn TermQuery, ident: RootBuiltinIdentifier) -> TermPtr {
        db.it_term(
            Self {
                path: TermEntityPath::root(ident.into()),
                ty: Ty::entity_ty_ty(db),
            }
            .into(),
        )
    }
}
