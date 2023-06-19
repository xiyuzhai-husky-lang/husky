use husky_entity_tree::ImplBlockNode;

use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeMemoizedFieldDeclarativeSignatureTemplate {
    pub impl_block: TypeImplBlockDeclarativeSignatureTemplate,
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for TypeMemoizedFieldDecl {
    type DeclarativeSignatureTemplate = TypeMemoizedFieldDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        ty_memoized_field_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_memoized_field_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeMemoizedFieldDecl,
) -> DeclarativeSignatureResult<TypeMemoizedFieldDeclarativeSignatureTemplate> {
    let impl_block_declarative_signature_template = decl
        .impl_block_node_id(db)
        .declarative_signature_template(db)?;
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let memo_ty = match decl.memo_ty(db) {
        Some(memo_ty) => declarative_term_region.expr_term(memo_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(TypeMemoizedFieldDeclarativeSignatureTemplate::new(
        db,
        impl_block_declarative_signature_template,
        memo_ty,
    ))
}

pub trait HasTypeMemoizedFieldDeclarativeSignatureTemplates: Copy {
    fn ty_memoized_field_declarative_signature_templates_map<'a>(
        self,
        db: &'a dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<
        &'a [(
            Ident,
            DeclarativeSignatureResult<SmallVecImpl<TypeMemoizedFieldDeclarativeSignatureTemplate>>,
        )],
    >;

    fn ty_memoized_field_declarative_signature_templates<'a>(
        self,
        db: &'a dyn DeclarativeSignatureDb,
        ident: Ident,
    ) -> DeclarativeSignatureResult<Option<&'a [TypeMemoizedFieldDeclarativeSignatureTemplate]>>
    {
        use vec_like::VecMapGetEntry;
        match self
            .ty_memoized_field_declarative_signature_templates_map(db)?
            .get_entry(ident)
        {
            Some((_, Ok(templates))) => Ok(Some(templates)),
            Some((_, Err(e))) => Err(*e),
            None => Ok(None),
        }
    }
}

impl HasTypeMemoizedFieldDeclarativeSignatureTemplates for TypePath {
    fn ty_memoized_field_declarative_signature_templates_map<'a>(
        self,
        db: &'a dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<
        &'a [(
            Ident,
            DeclarativeSignatureResult<SmallVecImpl<TypeMemoizedFieldDeclarativeSignatureTemplate>>,
        )],
    > {
        ty_memoized_field_declarative_signature_templates_map(db, self)
            .as_ref()
            .map(|v| v as &[_])
            .map_err(|e| *e)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar, return_ref)]
pub(crate) fn ty_memoized_field_declarative_signature_templates_map(
    db: &dyn DeclarativeSignatureDb,
    ty_path: TypePath,
) -> DeclarativeSignatureResult<
    IdentPairMap<
        DeclarativeSignatureResult<SmallVecImpl<TypeMemoizedFieldDeclarativeSignatureTemplate>>,
    >,
> {
    let item_decls_map = ty_path.item_decls_map(db)?;
    Ok(
        IdentPairMap::from_iter_assuming_no_repetitions(item_decls_map.iter().filter_map(
            |(ident, decls)| {
                match decls {
                    Ok(TypeItemDecls::MemoizedField(decls)) => Some((
                        *ident,
                        decls
                            .iter()
                            .copied()
                            .map(|decl| decl.declarative_signature_template(db))
                            .collect::<DeclarativeSignatureResult<SmallVecImpl<_>>>(),
                    )),
                    Ok(_) => None,
                    Err(_) => Some((*ident, Err(DeclarativeSignatureError::DeclError))),
                }
            },
        ))
        .expect("no repetitions"),
    )
}
