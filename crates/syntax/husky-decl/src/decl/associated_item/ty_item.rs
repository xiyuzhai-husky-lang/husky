mod associated_fn;
mod associated_ty;
mod associated_value;
mod memo;
mod method_fn;

pub use associated_fn::*;
pub use associated_ty::*;
pub use associated_value::*;
use husky_word::{Ident, IdentPairMap};
pub use memo::*;
pub use method_fn::*;

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

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn ty_item_decls<'a>(
    db: &'a dyn DeclDb,
    path: TypePath,
) -> EntityTreeBundleResult<IdentPairMap<Result<TypeItemDecl, ()>>> {
    let ty_items = db.ty_items(path)?;
    Ok(ty_items
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

    pub fn implicit_parameters<'a>(
        self,
        _db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
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
