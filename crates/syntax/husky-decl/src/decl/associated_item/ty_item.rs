mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;
use husky_ast::*;
use husky_entity_taxonomy::TypeItemKind;
use husky_word::{Ident, IdentPairMap};
use vec_like::VecMapGetEntry;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeItemRawDecl {
    AssociatedFn(TypeAssociatedFnRawDecl),
    MethodFn(TypeMethodFnRawDecl),
    AssociatedType(TypeAssociatedTypeRawDecl),
    AssociatedVal(TypeAssociatedValRawDecl),
    MemoizedField(TypeMemoizedFieldRawDecl),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeItemDecl {
    AssociatedFn(TypeAssociatedFnDecl),
    MethodFn(TypeMethodFnDecl),
    AssociatedType(TypeAssociatedTypeDecl),
    AssociatedVal(TypeAssociatedValDecl),
    MemoizedField(TypeMemoizedFieldDecl),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeItemDecls {
    AssociatedFn(SmallVecImpl<TypeAssociatedFnDecl>),
    MethodFn(SmallVecImpl<TypeMethodFnDecl>),
    MethodFunction(/* adhoc */),
    AssociatedType(SmallVecImpl<TypeAssociatedTypeDecl>),
    AssociatedVal(SmallVecImpl<TypeAssociatedValDecl>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldDecl>),
}

impl HasDecl for TypeItemPath {
    type Decl = TypeItemDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        Err(&DeclError::Original(OriginalDeclError::Deprecated))
        // todo!("deprecated")
        // self.parent_ty(db)
        //     .item_decls(db)
        //     .map_err(|_| todo!())?
        //     .get_entry(self.ident(db))
        //     .ok_or(&DeclError::Original(OriginalDeclError::NoSuchItem))?
        //     .1
        //     .map_err(|_| todo!())
    }
}

impl HasItemDecls for TypeItemPath {
    type ItemDecls = TypeItemDecls;

    fn item_decls<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, &'a Self::ItemDecls> {
        self.parent_ty(db)
            .item_decls_map(db)
            .map_err(|_| todo!())?
            .get_entry(self.ident(db))
            .ok_or(&DeclError::Original(OriginalDeclError::NoSuchItem))?
            .1
            .as_ref()
            .map_err(|_| todo!())
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn ty_item_decls_map<'a>(
    db: &'a dyn DeclDb,
    path: TypePath,
) -> EntityTreeBundleResult<IdentPairMap<Result<TypeItemDecls, ()>>> {
    let mut map = IdentPairMap::default();
    for (ident, ty_item) in path.items(db)?.iter().copied() {
        let ty_item_kind = match ty_item.associated_item_kind(db) {
            AssociatedItemKind::TypeItem(ty_item_kind) => ty_item_kind,
            _ => unreachable!(),
        };
        let result = map.get_mut_or_insert_with(ident, || {
            Ok(match ty_item_kind {
                TypeItemKind::MethodFn => TypeItemDecls::MethodFn(Default::default()),
                TypeItemKind::AssociatedFn => TypeItemDecls::AssociatedFn(Default::default()),
                TypeItemKind::AssociatedVal => TypeItemDecls::AssociatedVal(Default::default()),
                TypeItemKind::AssociatedType => TypeItemDecls::AssociatedType(Default::default()),
                TypeItemKind::MemoizedField => TypeItemDecls::MemoizedField(Default::default()),
            })
        });
        let Ok(decl) = ty_item.decl(db) else {
            *result = Err(());
            continue
        };
        let AssociatedItemDecl::TypeItem(decl) = decl else {
            unreachable!()
        };
        match result {
            Ok(decls) => match (decls, decl) {
                (TypeItemDecls::AssociatedFn(decls), TypeItemDecl::AssociatedFn(decl)) => {
                    decls.push(decl)
                }
                (TypeItemDecls::MethodFn(decls), TypeItemDecl::MethodFn(decl)) => decls.push(decl),
                (TypeItemDecls::AssociatedType(decls), TypeItemDecl::AssociatedType(decl)) => {
                    decls.push(decl)
                }
                (TypeItemDecls::AssociatedVal(decls), TypeItemDecl::AssociatedVal(decl)) => {
                    decls.push(decl)
                }
                (TypeItemDecls::MemoizedField(decls), TypeItemDecl::MemoizedField(decl)) => {
                    decls.push(decl)
                }
                _ => *result = Err(()), // error because of inconsistent type item kind
            },
            Err(_) => continue,
        }
    }
    Ok(map)
}

impl TypeItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TypeItemDecl::AssociatedFn(decl) => decl.ast_idx(db),
            TypeItemDecl::MethodFn(decl) => decl.ast_idx(db),
            TypeItemDecl::AssociatedType(decl) => decl.ast_idx(db),
            TypeItemDecl::AssociatedVal(decl) => decl.ast_idx(db),
            TypeItemDecl::MemoizedField(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        _db: &'a dyn DeclDb,
    ) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            TypeItemDecl::AssociatedFn(_) => todo!(),
            TypeItemDecl::MethodFn(_) => todo!(),
            TypeItemDecl::AssociatedType(_) => todo!(),
            TypeItemDecl::AssociatedVal(_) => todo!(),
            TypeItemDecl::MemoizedField(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TypeItemDecl::AssociatedFn(decl) => decl.expr_region(db),
            TypeItemDecl::MethodFn(decl) => decl.expr_region(db),
            TypeItemDecl::AssociatedType(decl) => decl.expr_region(db),
            TypeItemDecl::AssociatedVal(decl) => decl.expr_region(db),
            TypeItemDecl::MemoizedField(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<TypeItemPath> {
        match self {
            TypeItemDecl::AssociatedFn(decl) => decl.path(db),
            TypeItemDecl::MethodFn(decl) => decl.path(db),
            // decl.path(db),
            TypeItemDecl::AssociatedType(_) => todo!(),
            TypeItemDecl::AssociatedVal(_) => todo!(),
            TypeItemDecl::MemoizedField(decl) => decl.path(db),
        }
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_ty_item_decl(
        &self,
        ty_item_kind: TypeItemKind,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenStreamState,
    ) -> Result<TypeItemDecl, DeclError> {
        Ok(match ty_item_kind {
            TypeItemKind::MethodFn => self
                .parse_ty_method_decl(
                    ast_idx,
                    token_group_idx,
                    associated_item,
                    saved_stream_state,
                )?
                .into(),
            TypeItemKind::AssociatedFn => self
                .parse_ty_associated_fn_decl(
                    ast_idx,
                    token_group_idx,
                    associated_item,
                    saved_stream_state,
                )?
                .into(),
            TypeItemKind::MemoizedField => self
                .parse_ty_memo_decl(
                    ast_idx,
                    token_group_idx,
                    associated_item,
                    saved_stream_state,
                )?
                .into(),
            TypeItemKind::AssociatedVal => todo!(),
            TypeItemKind::AssociatedType => todo!(),
        })
    }
}

impl HasItemDeclsMap for TypePath {
    type ItemDecls = TypeItemDecls;

    fn item_decls_map<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Result<Self::ItemDecls, ()>)]> {
        match ty_item_decls_map(db, self) {
            Ok(ty_item_decls_map) => Ok(ty_item_decls_map),
            Err(e) => Err(e),
        }
    }
}
