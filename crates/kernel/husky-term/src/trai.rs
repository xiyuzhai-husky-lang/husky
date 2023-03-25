use super::*;

impl Term {
    pub fn satisfies_trai(self, db: &dyn TermDb, trai: Term) -> TermResult<bool> {
        todo!()
    }

    pub fn is_ty_clonable(self, db: &dyn TermDb) -> TermResult<bool> {
        todo!()
    }
    pub fn is_ty_copyable(self, db: &dyn TermDb) -> TermResult<bool> {
        todo!()
    }
    pub fn is_ty_defaultable(self, db: &dyn TermDb) -> TermResult<bool> {
        let Some(term_menu) = self.term_menu(db) else {
            return Ok(false);
        };
        self.satisfies_trai(db, term_menu.default_trai())
    }
}
