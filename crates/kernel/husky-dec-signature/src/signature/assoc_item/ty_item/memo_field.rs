use crate::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TypeMemoizedFieldDecTemplate {
    pub path: TypeItemPath,
    pub impl_block: TypeImplBlockDecTemplate,
    pub return_ty: DecTerm,
}

impl TypeMemoizedFieldDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeMemoizedFieldSynDecl,
    ) -> DecSignatureResult<TypeMemoizedFieldDecTemplate> {
        let impl_block_syn_dec_template = decl.impl_block_path(db).dec_template(db)?;
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let return_ty = match decl.return_ty(db) {
            Some(return_ty) => dec_term_region.expr_term(return_ty.syn_expr_idx())?,
            None => dec_term_menu.unit(),
        };
        Ok(TypeMemoizedFieldDecTemplate::new(
            db,
            path,
            impl_block_syn_dec_template,
            return_ty,
        ))
    }
}

// pub trait HasTypeMemoizedFieldDecTemplates: Copy {
//     fn ty_memo_field_dec_templates_map<'a>(
//         self,
//         db: &'a ::salsa::Db,
//     ) -> DecSignatureResult<
//         &'a [(
//             Ident,
//             DecSignatureResult<SmallVecImpl<TypeMemoizedFieldDecTemplate>>,
//         )],
//     >;

//     fn ty_memo_field_dec_templates<'a>(
//         self,
//         db: &'a ::salsa::Db,
//         ident: Ident,
//     ) -> DecSignatureResult<Option<&'a [TypeMemoizedFieldDecTemplate]>>
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

// impl HasTypeMemoizedFieldDecTemplates for TypePath {
//     fn ty_memo_field_dec_templates_map<'a>(
//         self,
//         db: &'a ::salsa::Db,
//     ) -> DecSignatureResult<
//         &'a [(
//             Ident,
//             DecSignatureResult<SmallVecImpl<TypeMemoizedFieldDecTemplate>>,
//         )],
//     > {
//         ty_memo_field_dec_templates_map(db, self)
//             .as_ref()
//             .map(|v| v as &[_])
//             .map_err(|e| *e)
//     }
// }

// #[salsa::tracked(jar = DecSignatureJar, return_ref)]
// pub(crate) fn ty_memo_field_dec_templates_map(
//     db: &::salsa::Db,
//     ty_path: TypePath,
// ) -> DecSignatureResult<
//     IdentPairMap<
//         DecSignatureResult<SmallVecImpl<TypeMemoizedFieldDecTemplate>>,
//     >,
// > {
//     let item_syn_decls_map = ty_path.item_syn_decls_map(db)?;
//     Ok(
//         IdentPairMap::from_iter_assuming_no_repetitions(item_syn_decls_map.iter().filter_map(
//             |(ident, decls)| {
//                 match decls {
//                     Ok(TypeItemDecls::MemoizedField(decls)) => Some((
//                         *ident,
//                         decls
//                             .iter()
//                             .copied()
//                             .map(|decl| decl.dec_template(db))
//                             .collect::<DecSignatureResult<SmallVecImpl<_>>>(),
//                     )),
//                     Ok(_) => None,
//                     Err(_) => Some((*ident, Err(DecSignatureError::DeclError))),
//                 }
//             },
//         ))
//         .expect("no repetitions"),
//     )
// }
