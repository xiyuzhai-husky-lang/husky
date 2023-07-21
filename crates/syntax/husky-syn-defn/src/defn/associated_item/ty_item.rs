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
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TypeItemNodeDefn {
    AssociatedFn(TypeAssociatedFnNodeDefn),
    MethodFn(TypeMethodFnNodeDefn),
    AssociatedType(TypeAssociatedTypeNodeDefn),
    AssociatedVal(TypeAssociatedValNodeDefn),
    MemoizedField(TypeMemoizedFieldNodeDefn),
}

impl TypeItemNodeDefn {
    pub fn node_path(self, db: &dyn DefnDb) -> TypeItemNodePath {
        match self {
            TypeItemNodeDefn::AssociatedFn(node_defn) => node_defn.node_path(db),
            TypeItemNodeDefn::MethodFn(node_defn) => node_defn.node_path(db),
            TypeItemNodeDefn::AssociatedType(node_defn) => node_defn.node_path(db),
            TypeItemNodeDefn::AssociatedVal(node_defn) => node_defn.node_path(db),
            TypeItemNodeDefn::MemoizedField(node_defn) => node_defn.node_path(db),
        }
    }

    pub fn node_decl(self, db: &dyn DefnDb) -> TypeItemNodeDecl {
        match self {
            TypeItemNodeDefn::AssociatedFn(node_defn) => node_defn.node_decl(db).into(),
            TypeItemNodeDefn::MethodFn(node_defn) => node_defn.node_decl(db).into(),
            TypeItemNodeDefn::AssociatedType(node_defn) => node_defn.node_decl(db).into(),
            TypeItemNodeDefn::AssociatedVal(node_defn) => node_defn.node_decl(db).into(),
            TypeItemNodeDefn::MemoizedField(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            TypeItemNodeDefn::AssociatedFn(node_defn) => node_defn.expr_region(db).into(),
            TypeItemNodeDefn::MethodFn(node_defn) => node_defn.expr_region(db).into(),
            TypeItemNodeDefn::AssociatedType(node_defn) => node_defn.expr_region(db).into(),
            TypeItemNodeDefn::AssociatedVal(node_defn) => node_defn.expr_region(db).into(),
            TypeItemNodeDefn::MemoizedField(node_defn) => node_defn.expr_region(db).into(),
        }
    }
}

impl HasNodeDefn for TypeItemNodePath {
    type NodeDefn = TypeItemNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        ty_item_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_item_node_defn(db: &dyn DefnDb, node_path: TypeItemNodePath) -> TypeItemNodeDefn {
    match node_path.node_decl(db) {
        TypeItemNodeDecl::AssociatedFn(node_decl) => {
            TypeAssociatedFnNodeDefn::new(db, node_path, node_decl).into()
        }
        TypeItemNodeDecl::MethodFn(node_decl) => {
            TypeMethodFnNodeDefn::new(db, node_path, node_decl).into()
        }
        TypeItemNodeDecl::AssociatedType(_) => todo!(),
        TypeItemNodeDecl::AssociatedVal(_) => todo!(),
        TypeItemNodeDecl::MemoizedField(node_decl) => {
            TypeMemoizedFieldNodeDefn::new(db, node_path, node_decl).into()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TypeItemDefn {
    AssociatedFn(TypeAssociatedFnDefn),
    MethodFn(TypeMethodFnDefn),
    AssociatedType(TypeAssociatedTypeDefn),
    AssociatedVal(TypeAssociatedValDefn),
    MemoizedField(TypeMemoizedFieldDefn),
}

impl TypeItemDefn {
    pub fn path(self, _db: &dyn DefnDb) -> AssociatedItemPath {
        todo!()
    }

    pub fn decl(self, db: &dyn DefnDb) -> TypeItemDecl {
        match self {
            TypeItemDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TypeItemDefn::MethodFn(defn) => defn.decl(db).into(),
            TypeItemDefn::AssociatedType(defn) => defn.decl(db).into(),
            TypeItemDefn::AssociatedVal(defn) => defn.decl(db).into(),
            TypeItemDefn::MemoizedField(defn) => defn.decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            TypeItemDefn::AssociatedFn(defn) => defn.expr_region(db).into(),
            TypeItemDefn::MethodFn(defn) => defn.expr_region(db).into(),
            TypeItemDefn::AssociatedType(defn) => defn.expr_region(db).into(),
            TypeItemDefn::AssociatedVal(defn) => defn.expr_region(db).into(),
            TypeItemDefn::MemoizedField(defn) => defn.expr_region(db).into(),
        }
    }
}

impl HasDefn for TypeItemPath {
    type Defn = TypeItemDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        ty_item_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_item_defn(db: &dyn DefnDb, path: TypeItemPath) -> DefnResult<TypeItemDefn> {
    let decl = path.decl(db)?;
    Ok(match decl {
        TypeItemDecl::AssociatedFn(decl) => TypeAssociatedFnDefn::new(db, path, decl)?.into(),
        TypeItemDecl::MethodFn(decl) => TypeMethodFnDefn::new(db, path, decl)?.into(),
        TypeItemDecl::AssociatedType(_) => todo!(),
        TypeItemDecl::AssociatedVal(_) => todo!(),
        TypeItemDecl::MemoizedField(decl) => TypeMemoizedFieldDefn::new(db, path, decl)?.into(),
    })
}
