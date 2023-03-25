use crate::*;

impl Term {
    pub fn satisfies_trai(self, db: &dyn TermDb, trai: Term) -> TermResult<bool> {
        trai_side_trai_for_ty_impl_blocks(db, trai);
        // todo!()
        Ok(true)
    }

    pub fn is_ty_clonable(self, db: &dyn TermDb) -> TermResult<bool> {
        let Some(term_menu) = self.term_menu(db) else {
            return Ok(false);
        };
        self.satisfies_trai(db, term_menu.clone_trai())
    }
    pub fn is_ty_copyable(self, db: &dyn TermDb) -> TermResult<bool> {
        let Some(term_menu) = self.term_menu(db) else {
            return Ok(false);
        };
        self.satisfies_trai(db, term_menu.copy_trai())
    }
    pub fn is_ty_defaultable(self, db: &dyn TermDb) -> TermResult<bool> {
        let Some(term_menu) = self.term_menu(db) else {
            return Ok(false);
        };
        self.satisfies_trai(db, term_menu.default_trai())
    }
}

pub(crate) fn trai_side_trai_for_ty_impl_blocks(db: &dyn TermDb, trai: Term) {
    // todo!()
}

#[test]
fn satisfies_trai_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let term_menu = db.term_menu(toolchain);
    assert!(term_menu.i8_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i16_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i32_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i64_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i128_ty_ontology().is_ty_clonable(&db).unwrap());
}
