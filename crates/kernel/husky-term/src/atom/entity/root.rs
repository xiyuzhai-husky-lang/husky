use husky_word::Identifier;

use crate::*;

impl Term {
    // pub(crate) fn std(db: &dyn TermDb, _menu2: &TermMenu2) -> Term {
    //     todo!()
    //     // Self::root_builtin_entity(db, Std, todo!())
    // }

    // pub(crate) fn core(db: &dyn TermDb, menu2: &TermMenu2) -> Term {
    //     todo!()
    //     // Self::root_builtin_entity(db, Core, menu2.module())
    // }

    pub(crate) fn root_builtin_entity(_db: &dyn TermDb, _ident: Identifier, _ty: Term) -> Term {
        todo!()
        // db.it_term(
        //     TermAtom::Entity {
        //         path: db.it_entity_path(EntityPath::root(ident.into())),
        //     }
        //     .into(),
        // )
    }
}
