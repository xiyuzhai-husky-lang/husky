use crate::*;
use husky_entity_tree::TraitForTypeImplBlock;
use smallvec::SmallVec;

impl Term {
    pub fn satisfies_trai(self, db: &dyn TermDb, trai: Term) -> TermResult<bool> {
        // todo: derives??
        Ok(
            search_among_impl_blocks(db, trai_side_trai_for_ty_impl_blocks(db, trai, self)?)?
                .is_some()
                || search_among_impl_blocks(db, ty_side_trai_for_ty_impl_blocks(db, trai, self)?)?
                    .is_some(),
        )
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

pub(crate) fn trai_side_trai_for_ty_impl_blocks<'a>(
    db: &'a dyn TermDb,
    trai: Term,
    ty: Term,
) -> TermResult<&'a [TraitForTypeImplBlock]> {
    let Some(trai_path) = trai.leading_trai_path(db) else {
        return Ok(&[])
    };
    let Some(ty_path) = ty.leading_ty_path(db) else {
        return Ok(&[])
    };
    match trai_side_trai_for_ty_impl_blocks_aux(db, trai_path, ty_path) {
        Ok(impl_blocks) => Ok(impl_blocks),
        Err(e) => Err(e.clone()),
    }
}

#[salsa::tracked(jar = TermJar, return_ref)]
pub(crate) fn trai_side_trai_for_ty_impl_blocks_aux<'a>(
    db: &'a dyn TermDb,
    trai_path: TraitPath,
    ty_path: TypePath,
) -> TermResult<SmallVec<[TraitForTypeImplBlock; 2]>> {
    Ok(db
        .entity_tree_bundle(trai_path.crate_path(db))?
        .trai_for_ty_impl_blocks_filtered(db, trai_path, ty_path)
        .collect())
}

pub(crate) fn ty_side_trai_for_ty_impl_blocks<'a>(
    db: &'a dyn TermDb,
    trai: Term,
    ty: Term,
) -> TermResult<&'a [TraitForTypeImplBlock]> {
    let Some(trai_path) = trai.leading_trai_path(db) else {
        return Ok(&[])
    };
    let Some(ty_path) = ty.leading_ty_path(db) else {
        return Ok(&[])
    };
    // ignore if they are from the same crate
    // because trait side would suffice
    if trai_path.crate_path(db) == ty_path.crate_path(db) {
        return Ok(&[]);
    }
    match ty_side_trai_for_ty_impl_blocks_aux(db, trai_path, ty_path) {
        Ok(impl_blocks) => Ok(impl_blocks),
        Err(e) => Err(e.clone()),
    }
}

#[salsa::tracked(jar = TermJar, return_ref)]
pub(crate) fn ty_side_trai_for_ty_impl_blocks_aux<'a>(
    db: &'a dyn TermDb,
    trai_path: TraitPath,
    ty_path: TypePath,
) -> TermResult<SmallVec<[TraitForTypeImplBlock; 2]>> {
    assert_ne!(trai_path.crate_path(db), ty_path.crate_path(db));
    Ok(db
        .entity_tree_bundle(ty_path.crate_path(db))?
        .trai_for_ty_impl_blocks_filtered(db, trai_path, ty_path)
        .collect())
}

#[inline(always)]
fn search_among_impl_blocks<'a>(
    db: &'a dyn TermDb,
    impl_blocks: &'a [TraitForTypeImplBlock],
) -> TermResult<Option<TraitForTypeImplBlock>> {
    for impl_block in impl_blocks.iter().copied() {
        todo!()
    }
    Ok(None)
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
