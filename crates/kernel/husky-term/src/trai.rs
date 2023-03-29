use crate::*;
use husky_decr::{Decr, HasDecrs};
use husky_entity_tree::TraitForTypeImplBlock;
use husky_signature::HasSignature;
use smallvec::SmallVec;

impl Term {
    pub fn satisfies_trai(self, db: &dyn TermDb, trai: Term) -> TermResult<bool> {
        Ok(self.trai_satisfiction(db, trai)?.is_some())
    }

    pub fn trai_satisfiction<'a>(
        self,
        db: &'a dyn TermDb,
        trai: Term,
    ) -> TermResult<Option<&'a TraitForTypeImplCard>> {
        let Some(trai_path) = trai.leading_trai_path(db) else {
            todo!()
        };
        let ty_path = self.leading_ty_path(db);
        if let Some(card) = search_among_impl_blocks(
            db,
            trai_path,
            ty_path,
            trai,
            self,
            trai_side_trai_for_ty_impl_block_cards(db, trai_path)?,
        )? {
            return Ok(Some(card));
        }
        if let Some(ty_path) = ty_path {
            if let Some(card) = search_among_impl_blocks(
                db,
                trai_path,
                Some(ty_path),
                trai,
                self,
                ty_side_trai_for_ty_impl_block_cards(db, trai_path, ty_path)?,
            )? {
                return Ok(Some(card));
            }
        }
        // todo: trait from context
        Ok(None)
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

pub(crate) fn trai_side_trai_for_ty_impl_block_cards<'a>(
    db: &'a dyn TermDb,
    trai_path: TraitPath,
) -> TermResult<&'a [TraitForTypeImplCard]> {
    match trai_side_trai_for_ty_impl_blocks_aux(db, trai_path) {
        Ok(impl_blocks) => Ok(impl_blocks),
        Err(e) => Err(e.clone()),
    }
}

#[salsa::tracked(jar = TermJar, return_ref)]
pub(crate) fn trai_side_trai_for_ty_impl_blocks_aux<'a>(
    db: &'a dyn TermDb,
    trai_path: TraitPath,
) -> TermResult<SmallVec<[TraitForTypeImplCard; 2]>> {
    db.entity_tree_bundle(trai_path.crate_path(db))?
        .trai_for_ty_impl_blocks_filtered_by_trai_path(db, trai_path)
        .map(|impl_block| TraitForTypeImplCard::from_impl_block(db, impl_block))
        .collect()
}

#[inline(always)]
pub(crate) fn ty_side_trai_for_ty_impl_block_cards<'a>(
    db: &'a dyn TermDb,
    trai_path: TraitPath,
    ty_path: TypePath,
) -> TermResult<&'a [TraitForTypeImplCard]> {
    match ty_side_trai_for_ty_impl_blocks_aux(db, ty_path) {
        Ok(cards) => Ok(cards),
        Err(e) => Err(e.clone()),
    }
}

#[salsa::tracked(jar = TermJar, return_ref)]
pub(crate) fn ty_side_trai_for_ty_impl_blocks_aux<'a>(
    db: &'a dyn TermDb,
    ty_path: TypePath,
) -> TermResult<Vec<TraitForTypeImplCard>> {
    let mut cards = vec![];
    for decr in ty_path.decrs(db)?.iter().copied() {
        TraitForTypeImplCard::collect_from_decr(db, decr, &mut cards)?
    }
    for impl_block in db
        .entity_tree_bundle(ty_path.crate_path(db))?
        .trai_for_ty_impl_blocks_filtered_by_ty_path(db, ty_path)
    {
        cards.push(TraitForTypeImplCard::from_impl_block(db, impl_block)?)
    }
    Ok(cards)
}

#[inline(always)]
fn search_among_impl_blocks<'a>(
    db: &'a dyn TermDb,
    trai_path: TraitPath,
    ty_path: Option<TypePath>,
    trai: Term,
    ty: Term,
    impl_block_cards: &'a [TraitForTypeImplCard],
) -> TermResult<Option<&'a TraitForTypeImplCard>> {
    for card in impl_block_cards.iter() {
        if trai == card.trai && ty == card.ty {
            return Ok(Some(card));
        }
        if trai_path != card.trai_path || ty_path != card.ty_path {
            continue;
        }
        p!(trai.debug(db), ty.debug(db), card.debug(db));
        todo!()
    }
    Ok(None)
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = TermDb, jar = TermJar)]
pub struct TraitForTypeImplCard {
    trai_path: TraitPath,
    ty_path: Option<TypePath>,
    trai: Term,
    ty: Term,
    src: TraitForTypeImplCardSource,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TermDb, jar = TermJar)]
pub enum TraitForTypeImplCardSource {
    ImplBlock(TraitForTypeImplBlock),
    Decr,
}
impl TraitForTypeImplCard {
    fn from_impl_block(db: &dyn TermDb, impl_block: TraitForTypeImplBlock) -> TermResult<Self> {
        trai_for_type_impl_card_from_impl_block(db, impl_block)
    }

    fn collect_from_decr<'a>(
        db: &'a dyn TermDb,
        decr: Decr,
        cards: &mut Vec<Self>,
    ) -> TermResult<()> {
        match decr {
            Decr::Derive(derive_decr) => {
                derive_decr.signature(db);
                todo!()
            }
        }
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn trai_for_type_impl_card_from_impl_block(
    db: &dyn TermDb,
    impl_block: TraitForTypeImplBlock,
) -> TermResult<TraitForTypeImplCard> {
    let signature = impl_block.signature(db)?;
    let trai = Term::from_raw_unchecked(db, signature.trai(db), TermTypeExpectation::Any)?;
    let ty = Term::ty_from_raw_unchecked(db, signature.ty(db))?;
    Ok(TraitForTypeImplCard {
        trai_path: trai.leading_trai_path(db).expect("should be valid trait"),
        ty_path: ty.leading_ty_path(db),
        trai,
        ty,
        src: TraitForTypeImplCardSource::ImplBlock(impl_block),
    })
}

#[test]
fn satisfies_trai_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let term_menu = db.term_menu(toolchain);
    p!(term_menu.i8_ty_ontology().is_ty_clonable(&db));
    assert!(term_menu.i8_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i16_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i32_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i64_ty_ontology().is_ty_clonable(&db).unwrap());
    assert!(term_menu.i128_ty_ontology().is_ty_clonable(&db).unwrap());
}
