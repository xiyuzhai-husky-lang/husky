use husky_entity_path::EntityPath;
use husky_word::RootBuiltinIdentifier;
use RootBuiltinIdentifier::*;

use crate::*;

impl TermEntity {
    pub(crate) fn std(db: &dyn TermQuery) -> TermPtr {
        Self::root_builtin_entity(db, Std)
    }

    pub(crate) fn root_builtin_entity(db: &dyn TermQuery, ident: RootBuiltinIdentifier) -> TermPtr {
        db.it_term(
            Self {
                path: db.it_root_entity_path(ident.into()),
                ty: Ty::entity_ty_ty(db),
            }
            .into(),
        )
    }
}
