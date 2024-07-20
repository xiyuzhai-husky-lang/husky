use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;
use husky_syn_decl::decl::assoc_item::trai_item::memo::TraitMemoizedFieldSynDecl;

#[salsa::interned]
pub struct TraitMemoizedFieldDecTemplate {
    pub path: TraitItemPath,
    pub return_ty: DecTerm,
}

impl TraitMemoizedFieldDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TraitItemPath,
        decl: TraitMemoizedFieldSynDecl,
    ) -> DecSignatureResult<TraitMemoizedFieldDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let return_ty = dec_term_region.expr_term(decl.return_ty(db).syn_expr_idx())?;
        Ok(TraitMemoizedFieldDecTemplate::new(db, path, return_ty))
    }
}

// pub trait HasTraitMemoizedFieldDecTemplates: Copy {
//     fn ty_memo_field_dec_templates_map<'a>(
//         self,
//         db: &'a ::salsa::Db,
//     ) -> DecSignatureResult<
//         &'a [(
//             Ident,
//             DecSignatureResult<SmallVecImpl<TraitMemoizedFieldDecTemplate>>,
//         )],
//     >;

//     fn ty_memo_field_dec_templates<'a>(
//         self,
//         db: &'a ::salsa::Db,
//         ident: Ident,
//     ) -> DecSignatureResult<Option<&'a [TraitMemoizedFieldDecTemplate]>>
//     {
//         use vec_like::VecMapGetEntry;
//         match self
//             .ty_memo_field_dec_templates_map(db)?
//             .get_entry(ident)
//         {
//             Some((_, Ok(templates))) => Ok(Some(templates)),
//             Some((_, Err(e))) => Err(*e),
//             None => Ok(None),
//         }
//     }
// }

// impl HasTraitMemoizedFieldDecTemplates for TraitPath {
//     fn ty_memo_field_dec_templates_map<'a>(
//         self,
//         db: &'a ::salsa::Db,
//     ) -> DecSignatureResult<
//         &'a [(
//             Ident,
//             DecSignatureResult<SmallVecImpl<TraitMemoizedFieldDecTemplate>>,
//         )],
//     > {
//         ty_memo_field_dec_templates_map(db, self)
//             .as_ref()
//             .map(|v| v as &[_])
//             .map_err(|e| *e)
//     }
// }

// #[salsa::tracked(return_ref)]
// pub(crate) fn ty_memo_field_dec_templates_map(
//     db: &::salsa::Db,
//     ty_path: TraitPath,
// ) -> DecSignatureResult<
//     IdentPairMap<
//         DecSignatureResult<SmallVecImpl<TraitMemoizedFieldDecTemplate>>,
//     >,
// > {
//     let item_syn_decls_map = ty_path.item_syn_decls_map(db)?;
//     Ok(
//         IdentPairMap::from_iter_assuming_no_repetitions(item_syn_decls_map.iter().filter_map(
//             |(ident, decls)| {
//                 match decls {
//                     Ok(TraitItemDecls::MemoizedField(decls)) => Some((
//                         *ident,
//                         decls
//                             .iter()
//                             .copied()
//                             .map(|decl| decl.dec_template(db))
//                             .collect::<DecSignatureResult<SmallVecImpl<_>>>(),
//                     )),
//                     Ok(_) => None,
//                     Err(_) => Some((*ident, Err(DecSignatureError::SynDeclError))),
//                 }
//             },
//         ))
//         .expect("no repetitions"),
//     )
// }
