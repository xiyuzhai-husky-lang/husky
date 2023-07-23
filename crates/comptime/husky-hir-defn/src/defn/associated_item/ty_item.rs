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
use husky_entity_path::AssociatedItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TypeItemHirDefn {
    AssociatedFn(TypeAssociatedFnHirDefn),
    MethodFn(TypeMethodFnHirDefn),
    AssociatedType(TypeAssociatedTypeHirDefn),
    AssociatedVal(TypeAssociatedValHirDefn),
    MemoizedField(TypeMemoizedFieldHirDefn),
}

impl TypeItemHirDefn {
    pub fn path(self, _db: &dyn HirDefnDb) -> AssociatedItemPath {
        todo!()
    }

    pub fn hir_decl(self, db: &dyn HirDefnDb) -> TypeItemHirDecl {
        match self {
            TypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            TypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.hir_expr_region(db).into(),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.hir_expr_region(db).into(),
            TypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.hir_expr_region(db).into(),
            TypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.hir_expr_region(db).into(),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.hir_expr_region(db).into(),
        }
    }
}

impl HasHirDefn for TypeItemPath {
    type HirDefn = TypeItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        ty_item_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn ty_item_hir_defn(db: &dyn HirDefnDb, path: TypeItemPath) -> TypeItemHirDefn {
    match path.hir_decl(db) {
        TypeItemHirDecl::AssociatedFn(hir_decl) => {
            TypeAssociatedFnHirDefn::new(db, path, hir_decl).into()
        }
        TypeItemHirDecl::MethodFn(hir_decl) => TypeMethodFnHirDefn::new(db, path, hir_decl).into(),
        TypeItemHirDecl::AssociatedType(_) => todo!(),
        TypeItemHirDecl::AssociatedVal(_) => todo!(),
        TypeItemHirDecl::MemoizedField(hir_decl) => {
            TypeMemoizedFieldHirDefn::new(db, path, hir_decl).into()
        }
    }
}
