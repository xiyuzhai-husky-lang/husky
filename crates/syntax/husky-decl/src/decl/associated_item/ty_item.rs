mod assoc_ty;
mod assoc_val;
mod function;
mod memo;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
use husky_word::{Ident, IdentPairMap};
pub use memo::*;
pub use method::*;

use crate::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeItemDecl {
    Function(TypeAssociatedFunctionDecl),
    Method(TypeMethodDecl),
    ExternType(TypeAssociatedTypeDecl),
    Value(TypeAssociatedValueDecl),
    Memo(TypeMemoDecl),
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn ty_item_decls<'a>(
    db: &'a dyn DeclDb,
    path: TypePath,
) -> EntityTreeCrateBundleResult<IdentPairMap<Result<TypeItemDecl, ()>>> {
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
            TypeItemDecl::Function(decl) => decl.ast_idx(db),
            TypeItemDecl::Method(decl) => decl.ast_idx(db),
            TypeItemDecl::ExternType(decl) => decl.ast_idx(db),
            TypeItemDecl::Value(decl) => decl.ast_idx(db),
            TypeItemDecl::Memo(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        _db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self {
            TypeItemDecl::Function(_) => todo!(),
            TypeItemDecl::Method(_) => todo!(),
            TypeItemDecl::ExternType(_) => todo!(),
            TypeItemDecl::Value(_) => todo!(),
            TypeItemDecl::Memo(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TypeItemDecl::Function(defn) => defn.expr_region(db),
            TypeItemDecl::Method(defn) => defn.expr_region(db),
            TypeItemDecl::ExternType(defn) => defn.expr_region(db),
            TypeItemDecl::Value(defn) => defn.expr_region(db),
            TypeItemDecl::Memo(defn) => defn.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<TypeItemPath> {
        match self {
            TypeItemDecl::Function(_) => todo!(),
            TypeItemDecl::Method(defn) => defn.path(db),
            TypeItemDecl::ExternType(_) => todo!(),
            TypeItemDecl::Value(_) => todo!(),
            TypeItemDecl::Memo(defn) => defn.path(db),
        }
    }
}
