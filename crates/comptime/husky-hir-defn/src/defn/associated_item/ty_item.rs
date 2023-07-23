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
            TypeItemHirDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TypeItemHirDefn::MethodFn(defn) => defn.decl(db).into(),
            TypeItemHirDefn::AssociatedType(defn) => defn.decl(db).into(),
            TypeItemHirDefn::AssociatedVal(defn) => defn.decl(db).into(),
            TypeItemHirDefn::MemoizedField(defn) => defn.decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            TypeItemHirDefn::AssociatedFn(defn) => defn.expr_region(db).into(),
            TypeItemHirDefn::MethodFn(defn) => defn.expr_region(db).into(),
            TypeItemHirDefn::AssociatedType(defn) => defn.expr_region(db).into(),
            TypeItemHirDefn::AssociatedVal(defn) => defn.expr_region(db).into(),
            TypeItemHirDefn::MemoizedField(defn) => defn.expr_region(db).into(),
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
    let decl = path.syn_decl(db)?;
    Ok(match decl {
        TypeItemDecl::AssociatedFn(decl) => TypeAssociatedFnHirDefn::new(db, path, decl)?.into(),
        TypeItemDecl::MethodFn(decl) => TypeMethodFnHirDefn::new(db, path, decl)?.into(),
        TypeItemDecl::AssociatedType(_) => todo!(),
        TypeItemDecl::AssociatedVal(_) => todo!(),
        TypeItemDecl::MemoizedField(decl) => TypeMemoizedFieldHirDefn::new(db, path, decl)?.into(),
    })
}
