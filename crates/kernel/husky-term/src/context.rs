use std::sync::Arc;

use crate::*;

pub struct TermContext<'a> {
    db: &'a dyn TermQuery,
    menu: Arc<TermMenu>,
}

impl<'a> TermContext<'a> {
    pub fn new(db: &'a dyn TermQuery) -> Self {
        Self {
            db,
            menu: db.term_menu(),
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
