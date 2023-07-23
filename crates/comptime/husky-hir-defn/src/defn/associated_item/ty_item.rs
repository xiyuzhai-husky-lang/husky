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
pub enum TypeItemHirNodeDefn {
    AssociatedFn(TypeAssociatedFnHirNodeDefn),
    MethodFn(TypeMethodFnHirNodeDefn),
    AssociatedType(TypeAssociatedTypeHirNodeDefn),
    AssociatedVal(TypeAssociatedValHirNodeDefn),
    MemoizedField(TypeMemoizedFieldHirNodeDefn),
}

impl TypeItemHirNodeDefn {
    pub fn syn_node_path(self, db: &dyn HirDefnDb) -> TypeItemHirNodePath {
        match self {
            TypeItemHirNodeDefn::AssociatedFn(syn_node_defn) => syn_node_defn.syn_node_path(db),
            TypeItemHirNodeDefn::MethodFn(syn_node_defn) => syn_node_defn.syn_node_path(db),
            TypeItemHirNodeDefn::AssociatedType(syn_node_defn) => syn_node_defn.syn_node_path(db),
            TypeItemHirNodeDefn::AssociatedVal(syn_node_defn) => syn_node_defn.syn_node_path(db),
            TypeItemHirNodeDefn::MemoizedField(syn_node_defn) => syn_node_defn.syn_node_path(db),
        }
    }

    pub fn syn_node_decl(self, db: &dyn HirDefnDb) -> TypeItemNodeDecl {
        match self {
            TypeItemHirNodeDefn::AssociatedFn(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TypeItemHirNodeDefn::MethodFn(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeItemHirNodeDefn::AssociatedType(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TypeItemHirNodeDefn::AssociatedVal(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TypeItemHirNodeDefn::MemoizedField(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
        }
    }

    pub fn expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            TypeItemHirNodeDefn::AssociatedFn(syn_node_defn) => {
                syn_node_defn.expr_region(db).into()
            }
            TypeItemHirNodeDefn::MethodFn(syn_node_defn) => syn_node_defn.expr_region(db).into(),
            TypeItemHirNodeDefn::AssociatedType(syn_node_defn) => {
                syn_node_defn.expr_region(db).into()
            }
            TypeItemHirNodeDefn::AssociatedVal(syn_node_defn) => {
                syn_node_defn.expr_region(db).into()
            }
            TypeItemHirNodeDefn::MemoizedField(syn_node_defn) => {
                syn_node_defn.expr_region(db).into()
            }
        }
    }
}

impl HasHirNodeDefn for TypeItemHirNodePath {
    type NodeDefn = TypeItemHirNodeDefn;

    fn syn_node_defn(self, db: &dyn HirDefnDb) -> Self::NodeDefn {
        ty_item_syn_node_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn ty_item_syn_node_defn(
    db: &dyn HirDefnDb,
    syn_node_path: TypeItemHirNodePath,
) -> TypeItemHirNodeDefn {
    match syn_node_path.syn_node_decl(db) {
        TypeItemNodeDecl::AssociatedFn(syn_node_decl) => {
            TypeAssociatedFnHirNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeItemNodeDecl::MethodFn(syn_node_decl) => {
            TypeMethodFnHirNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeItemNodeDecl::AssociatedType(_) => todo!(),
        TypeItemNodeDecl::AssociatedVal(_) => todo!(),
        TypeItemNodeDecl::MemoizedField(syn_node_decl) => {
            TypeMemoizedFieldHirNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
    }
}

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

    pub fn decl(self, db: &dyn HirDefnDb) -> TypeItemDecl {
        match self {
            TypeItemHirDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TypeItemHirDefn::MethodFn(defn) => defn.decl(db).into(),
            TypeItemHirDefn::AssociatedType(defn) => defn.decl(db).into(),
            TypeItemHirDefn::AssociatedVal(defn) => defn.decl(db).into(),
            TypeItemHirDefn::MemoizedField(defn) => defn.decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
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

    fn syn_defn(self, db: &dyn HirDefnDb) -> HirDefnResult<Self::HirDefn> {
        ty_item_syn_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn ty_item_syn_defn(
    db: &dyn HirDefnDb,
    path: TypeItemPath,
) -> HirDefnResult<TypeItemHirDefn> {
    let decl = path.decl(db)?;
    Ok(match decl {
        TypeItemDecl::AssociatedFn(decl) => TypeAssociatedFnHirDefn::new(db, path, decl)?.into(),
        TypeItemDecl::MethodFn(decl) => TypeMethodFnHirDefn::new(db, path, decl)?.into(),
        TypeItemDecl::AssociatedType(_) => todo!(),
        TypeItemDecl::AssociatedVal(_) => todo!(),
        TypeItemDecl::MemoizedField(decl) => TypeMemoizedFieldHirDefn::new(db, path, decl)?.into(),
    })
}
