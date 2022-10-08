use husky_word::RootIdentifier;

use crate::*;

impl TermEntity {
    pub(crate) fn i32(db: &dyn TermQuery) -> TermPtr {
        db.it_term(
            Self {
                path: TermEntityPath::root(RootIdentifier::I32.into()),
                ty: Ty::entity_ty_ty(db),
            }
            .into(),
        )
    }

    pub(crate) fn i64(db: &dyn TermQuery) -> TermPtr {
        db.it_term(
            Self {
                path: TermEntityPath::root(RootIdentifier::I64.into()),
                ty: Ty::entity_ty_ty(db),
            }
            .into(),
        )
    }
}
