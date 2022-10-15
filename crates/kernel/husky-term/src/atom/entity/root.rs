use husky_entity_path::EntityPath;
use husky_word::RootBuiltinIdentifier;
use RootBuiltinIdentifier::*;

use crate::*;

impl Term {
    pub(crate) fn std(db: &dyn TermDb, menu2: &TermMenu2) -> TermPtr {
        Self::root_builtin_entity(db, Std, todo!())
    }

    pub(crate) fn core(db: &dyn TermDb, menu2: &TermMenu2) -> TermPtr {
        Self::root_builtin_entity(db, Core, todo!())
    }

    pub(crate) fn root_builtin_entity(
        db: &dyn TermDb,
        ident: RootBuiltinIdentifier,
        ty: Ty,
    ) -> TermPtr {
        db.it_term(
            TermAtom {
                variant: TermAtomVariant::Entity {
                    path: db.it_entity_path(EntityPath::root(ident.into())),
                },
                ty_itd: Some(ty),
            }
            .into(),
        )
    }
}
