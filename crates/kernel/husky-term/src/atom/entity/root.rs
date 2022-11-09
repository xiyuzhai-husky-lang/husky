use husky_entity_path::EntityPath;
use husky_word::RootBuiltinIdentifier;
use RootBuiltinIdentifier::*;

use crate::*;

impl TermOwned {
    pub(crate) fn std(db: &dyn TermDb, menu2: &TermMenu2) -> TermItd {
        Self::root_builtin_entity(db, Std, todo!())
    }

    pub(crate) fn core(db: &dyn TermDb, menu2: &TermMenu2) -> TermItd {
        Self::root_builtin_entity(db, Core, menu2.module())
    }

    pub(crate) fn root_builtin_entity(
        db: &dyn TermDb,
        ident: RootBuiltinIdentifier,
        ty: Ty,
    ) -> TermItd {
        db.it_term(
            TermAtom::Entity {
                path: db.it_entity_path(EntityPath::root(ident.into())),
            }
            .into(),
        )
    }
}
