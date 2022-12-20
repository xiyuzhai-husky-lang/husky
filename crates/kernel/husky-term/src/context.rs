use husky_entity_path::EntityPath;

use crate::*;

pub struct TermContext<'a> {
    pub(crate) db: &'a dyn TermDb,
    menu: &'a TermMenu,
}

impl<'a> TermContext<'a> {
    #[inline(always)]
    pub fn new(db: &'a dyn TermDb, menu: &'a TermMenu) -> Self {
        Self { db, menu }
    }

    pub fn from_provider<T>(provider: &T) -> Self
    where
        T: ProvideTermContext<'a> + ?Sized,
    {
        Self {
            db: provider.term_db(),
            menu: provider.term_menu(),
        }
    }

    pub fn entity_ty(&self, _entity_path: EntityPath) -> TermResult<Term> {
        todo!()
        // let decl = self.db.ask_decl(entity_path)?;
        // Ok(match entity_path.opt_parent() {
        //     Some(_) => todo!(),
        //     None => match *decl {
        //         Decl::Module => self.menu.module(),
        //         Decl::Term(ref ty_decl) => {
        //             if ty_decl.parameters().len() == 0 {
        //                 self.menu.ty0()
        //             } else {
        //                 todo!()
        //             }
        //         }
        //     },
        // })
    }
}

impl<'a> TermContext<'a> {
    // pub fn ty_family(&self, ty: Term) -> TermResult<TyFamily> {
    //     Ok(self.db.ty_decl(ty)?.ty_family())
    // }
}

pub trait ProvideTermContext<'a> {
    fn term_db(&self) -> &'a dyn TermDb;
    fn term_menu(&self) -> &'a TermMenu;
    fn curry(&self, curry_kind: TermCurryVariant, x: Term, y: Term) -> TermResult<Term> {
        let ctx = TermContext::from_provider(self);
        ctx.curry(curry_kind, x, y)
    }
}
