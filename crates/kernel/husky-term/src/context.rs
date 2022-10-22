use std::sync::Arc;

use crate::*;

pub struct TermContext<'a> {
    db: &'a dyn TermDb,
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
}

impl<'a> InternTerm for TermContext<'a> {
    fn term_itr(&self) -> &TermInterner {
        self.db.term_itr()
    }
}

impl<'a> TermContext<'a> {
    pub fn ty_family(&self, ty: Ty) -> TermResult<TyFamily> {
        Ok(self.db.ty_decl(ty)?.ty_family())
    }
}

pub trait ProvideTermContext<'a> {
    fn term_db(&self) -> &'a dyn TermDb;
    fn term_menu(&self) -> &'a TermMenu;
    fn curry(&self, curry_kind: TermCurryKind, x: Ty, y: Ty) -> TermResult<Ty> {
        let ctx = TermContext::from_provider(self);
        ctx.curry(curry_kind, x, y)
    }
}
