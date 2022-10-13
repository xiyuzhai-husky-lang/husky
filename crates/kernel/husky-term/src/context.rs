use std::sync::Arc;

use crate::*;

pub(crate) struct TermContext<'a> {
    db: &'a dyn TermDb,
    menu: &'a TermMenu,
}

impl<'a> TermContext<'a> {
    pub fn new(db: &'a dyn TermDb, menu: &'a TermMenu) -> Self {
        Self { db, menu }
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
        let ctx: TermContext = self.into();
        ctx.curry(curry_kind, x, y)
    }
}

impl<'a, T> From<&T> for TermContext<'a>
where
    T: ProvideTermContext<'a> + ?Sized,
{
    fn from(provider: &T) -> Self {
        Self {
            db: provider.term_db(),
            menu: provider.term_menu(),
        }
    }
}
