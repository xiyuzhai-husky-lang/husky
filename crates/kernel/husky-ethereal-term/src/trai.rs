use crate::*;
use husky_declarative_signature::{
    HasDeclarativeSignatureTemplate, TypeDeclarativeSignatureTemplate,
};
use husky_entity_syn_tree::TraitForTypeImplBlockNode;
use husky_syn_attr::{Attr, HasAttrs};
use smallvec::SmallVec;

impl EtherealTerm {
    pub fn satisfies_trai(self, db: &::salsa::Db, trai: EtherealTerm) -> EtherealTermResult<bool> {
        Ok(self.trai_satisfiction(db, trai)?.is_some())
    }

    pub fn trai_satisfiction<'a>(
        self,
        db: &'a ::salsa::Db,
        trai: EtherealTerm,
    ) -> EtherealTermResult<Option<&'a TraitForTypeImplTemplate>> {
        let Some(trai_path) = trai.leading_trai_path(db) else {
            todo!()
        };
        let ty_path = self.leading_ty_path(db);
        if let Some(template) = search_among_impl_blocks(
            db,
            trai_path,
            ty_path,
            trai,
            self,
            trai_side_trai_for_ty_impl_block_templates(db, trai_path)?,
        )? {
            return Ok(Some(template));
        }
        if let Some(ty_path) = ty_path {
            if let Some(template) = search_among_impl_blocks(
                db,
                trai_path,
                Some(ty_path),
                trai,
                self,
                ty_side_trai_for_ty_impl_block_templates(db, trai_path, ty_path)?,
            )? {
                return Ok(Some(template));
            }
        }
        // todo: trait from context
        Ok(None)
    }

    pub fn is_ty_clonable(self, db: &::salsa::Db) -> EtherealTermResult<bool> {
        let Some(term_menu) = self.term_menu(db) else {
            return Ok(false);
        };
        self.satisfies_trai(db, term_menu.clone_trai())
    }
    pub fn is_ty_copyable(self, db: &::salsa::Db) -> EtherealTermResult<bool> {
        let Some(term_menu) = self.term_menu(db) else {
            return Ok(false);
        };
        self.satisfies_trai(db, term_menu.copy_trai())
    }
    pub fn is_ty_defaultable(self, db: &::salsa::Db) -> EtherealTermResult<bool> {
        let Some(term_menu) = self.term_menu(db) else {
            return Ok(false);
        };
        self.satisfies_trai(db, term_menu.default_trai())
    }
}

pub(crate) fn trai_side_trai_for_ty_impl_block_templates<'a>(
    db: &'a ::salsa::Db,
    trai_path: TraitPath,
) -> EtherealTermResult<&'a [TraitForTypeImplTemplate]> {
    match trai_side_trai_for_ty_impl_blocks_aux(db, trai_path) {
        Ok(impl_blocks) => Ok(impl_blocks),
        Err(e) => Err(e.clone()),
    }
}

#[salsa::tracked(jar = EtherealTermJar, return_ref)]
pub(crate) fn trai_side_trai_for_ty_impl_blocks_aux<'a>(
    db: &'a ::salsa::Db,
    trai_path: TraitPath,
) -> EtherealTermResult<SmallVec<[TraitForTypeImplTemplate; 2]>> {
    db.item_tree_bundle(trai_path.crate_path(db))?
        .trai_for_ty_impl_block_paths_filtered_by_trai_path(db, trai_path)
        .map(|impl_block| impl_block.template(db))
        .collect()
}

#[inline(always)]
pub(crate) fn ty_side_trai_for_ty_impl_block_templates<'a>(
    db: &'a ::salsa::Db,
    trai_path: TraitPath,
    ty_path: TypePath,
) -> EtherealTermResult<&'a [TraitForTypeImplTemplate]> {
    match ty_side_trai_for_ty_impl_blocks_aux(db, ty_path) {
        Ok(templates) => Ok(templates),
        Err(e) => Err(e.clone()),
    }
}

#[salsa::tracked(jar = EtherealTermJar, return_ref)]
pub(crate) fn ty_side_trai_for_ty_impl_blocks_aux<'a>(
    db: &'a ::salsa::Db,
    ty_path: TypePath,
) -> EtherealTermResult<Vec<TraitForTypeImplTemplate>> {
    let mut templates = vec![];
    for attr in ty_path.attrs(db)?.iter().copied() {
        TraitForTypeImplTemplate::collect_from_attr(db, ty_path, attr, &mut templates)?
    }
    for path in db
        .item_tree_bundle(ty_path.crate_path(db))?
        .trai_for_ty_impl_block_paths_filtered_by_ty_path(db, ty_path)
    {
        templates.push(path.template(db)?)
    }
    Ok(templates)
}

#[inline(always)]
fn search_among_impl_blocks<'a>(
    db: &'a ::salsa::Db,
    trai_path: TraitPath,
    ty_path: Option<TypePath>,
    trai: EtherealTerm,
    ty: EtherealTerm,
    impl_block_templates: &'a [TraitForTypeImplTemplate],
) -> EtherealTermResult<Option<&'a TraitForTypeImplTemplate>> {
    for template in impl_block_templates.iter() {
        if trai == template.trai() && ty == template.ty() {
            return Ok(Some(template));
        }
        if trai_path != template.trai_path() || ty_path != template.ty_path() {
            continue;
        }
        use salsa::DisplayWithDb;
        p!(trai.debug(db), ty.display(db), template.debug(db));
        todo!()
    }
    Ok(None)
}

#[test]
fn satisfies_trai_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let term_menu = db.ethereal_term_menu(toolchain);
    assert!(term_menu.i8_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i16_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i32_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i64_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i128_ty_ontology().is_ty_clonable(&db).unwrap());
}
