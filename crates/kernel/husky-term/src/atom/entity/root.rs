use husky_entity_path::EntityPath;
use husky_identifier::Identifier;

use crate::*;

impl TermData {
    pub(crate) fn std(db: &dyn TermDb, _menu2: &TermMenu2) -> Term {
        todo!()
        // Self::root_builtin_entity(db, Std, todo!())
    }

    pub(crate) fn core(db: &dyn TermDb, menu2: &TermMenu2) -> Term {
        todo!()
        // Self::root_builtin_entity(db, Core, menu2.module())
    }

    pub(crate) fn root_builtin_entity(db: &dyn TermDb, ident: Identifier, _ty: Ty) -> Term {
        todo!()
        // db.it_term(
        //     TermAtom::Entity {
        //         path: db.it_entity_path(EntityPath::root(ident.into())),
        //     }
        //     .into(),
        // )
    }
}
