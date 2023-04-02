mod associated_fn;
mod associated_ty;
mod associated_value;
mod memo;
mod method_fn;

pub use associated_fn::*;
pub use associated_ty::*;
pub use associated_value::*;
use husky_entity_taxonomy::TypeItemKind;
use husky_word::{Ident, IdentPairMap};
pub use memo::*;
pub use method_fn::*;
use vec_like::VecMapGetEntry;

use crate::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeItemDecl {
    AssociatedFn(TypeAssociatedFnDecl),
    MethodFn(TypeMethodFnDecl),
    AssociatedType(TypeAssociatedTypeDecl),
    AssociatedValue(TypeAssociatedValueDecl),
    Memo(TypeMemoDecl),
}

impl HasDecl for TypeItemPath {
    type Decl = TypeItemDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        self.parent_ty(db)
            .item_decls(db)
            .map_err(|_| todo!())?
            .get_entry(self.ident(db))
            .ok_or(&DeclError::Original(OriginalDeclError::NoSuchItem))?
            .1
            .map_err(|_| todo!())
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn ty_item_decls<'a>(
    db: &'a dyn DeclDb,
    path: TypePath,
) -> EntityTreeBundleResult<IdentPairMap<Result<TypeItemDecl, ()>>> {
    Ok(path
        .items(db)?
        .iter()
        .copied()
        .map(|(ident, ty_item)| -> (Ident, Result<TypeItemDecl, ()>) {
            (
                ident,
                match associated_item_decl(db, ty_item) {
                    Ok(AssociatedItemDecl::TypeItem(decl)) => Ok(*decl),
                    Ok(_) => unreachable!(), // todo: reduce this
                    Err(_) => Err(()),
                },
            )
        })
        .collect())
}

impl TypeItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TypeItemDecl::AssociatedFn(decl) => decl.ast_idx(db),
            TypeItemDecl::MethodFn(decl) => decl.ast_idx(db),
            TypeItemDecl::AssociatedType(decl) => decl.ast_idx(db),
            TypeItemDecl::AssociatedValue(decl) => decl.ast_idx(db),
            TypeItemDecl::Memo(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        match self {
            TypeItemDecl::AssociatedFn(_) => todo!(),
            TypeItemDecl::MethodFn(_) => todo!(),
            TypeItemDecl::AssociatedType(_) => todo!(),
            TypeItemDecl::AssociatedValue(_) => todo!(),
            TypeItemDecl::Memo(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TypeItemDecl::AssociatedFn(decl) => decl.expr_region(db),
            TypeItemDecl::MethodFn(decl) => decl.expr_region(db),
            TypeItemDecl::AssociatedType(decl) => decl.expr_region(db),
            TypeItemDecl::AssociatedValue(decl) => decl.expr_region(db),
            TypeItemDecl::Memo(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<TypeItemPath> {
        match self {
            TypeItemDecl::AssociatedFn(decl) => decl.path(db),
            TypeItemDecl::MethodFn(decl) => decl.path(db),
            // decl.path(db),
            TypeItemDecl::AssociatedType(_) => todo!(),
            TypeItemDecl::AssociatedValue(_) => todo!(),
            TypeItemDecl::Memo(decl) => decl.path(db),
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
        saved_stream_state: TokenIdx,
    ) -> Result<AssociatedItemDecl, DeclError> {
        Ok(AssociatedItemDecl::TypeItem(match ty_item_kind {
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
            TypeItemKind::Memo => self
                .parse_ty_memo_decl(
                    ast_idx,
                    token_group_idx,
                    associated_item,
                    saved_stream_state,
                )?
                .into(),
            TypeItemKind::AssociatedVar => todo!(),
        }))
    }
}

impl HasItemDecls for TypePath {
    type ItemDecl = TypeItemDecl;

    fn item_decls<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Result<TypeItemDecl, ()>)]> {
        match ty_item_decls(db, self) {
            Ok(ty_item_decls) => Ok(ty_item_decls),
            Err(e) => Err(e),
        }
    }
}
