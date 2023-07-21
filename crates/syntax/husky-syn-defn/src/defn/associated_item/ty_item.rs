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
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TypeItemSynNodeDefn {
    AssociatedFn(TypeAssociatedFnNodeDefn),
    MethodFn(TypeMethodFnNodeDefn),
    AssociatedType(TypeAssociatedTypeNodeDefn),
    AssociatedVal(TypeAssociatedValNodeDefn),
    MemoizedField(TypeMemoizedFieldNodeDefn),
}

impl TypeItemSynNodeDefn {
    pub fn node_path(self, db: &dyn SynDefnDb) -> TypeItemSynNodePath {
        match self {
            TypeItemSynNodeDefn::AssociatedFn(node_defn) => node_defn.node_path(db),
            TypeItemSynNodeDefn::MethodFn(node_defn) => node_defn.node_path(db),
            TypeItemSynNodeDefn::AssociatedType(node_defn) => node_defn.node_path(db),
            TypeItemSynNodeDefn::AssociatedVal(node_defn) => node_defn.node_path(db),
            TypeItemSynNodeDefn::MemoizedField(node_defn) => node_defn.node_path(db),
        }
    }

    pub fn node_decl(self, db: &dyn SynDefnDb) -> TypeItemNodeDecl {
        match self {
            TypeItemSynNodeDefn::AssociatedFn(node_defn) => node_defn.node_decl(db).into(),
            TypeItemSynNodeDefn::MethodFn(node_defn) => node_defn.node_decl(db).into(),
            TypeItemSynNodeDefn::AssociatedType(node_defn) => node_defn.node_decl(db).into(),
            TypeItemSynNodeDefn::AssociatedVal(node_defn) => node_defn.node_decl(db).into(),
            TypeItemSynNodeDefn::MemoizedField(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            TypeItemSynNodeDefn::AssociatedFn(node_defn) => node_defn.expr_region(db).into(),
            TypeItemSynNodeDefn::MethodFn(node_defn) => node_defn.expr_region(db).into(),
            TypeItemSynNodeDefn::AssociatedType(node_defn) => node_defn.expr_region(db).into(),
            TypeItemSynNodeDefn::AssociatedVal(node_defn) => node_defn.expr_region(db).into(),
            TypeItemSynNodeDefn::MemoizedField(node_defn) => node_defn.expr_region(db).into(),
        }
    }
}

impl HasSynNodeDefn for TypeItemSynNodePath {
    type NodeDefn = TypeItemSynNodeDefn;

    fn node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        ty_item_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_item_node_defn(
    db: &dyn SynDefnDb,
    node_path: TypeItemSynNodePath,
) -> TypeItemSynNodeDefn {
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
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TypeItemSynDefn {
    AssociatedFn(TypeAssociatedFnDefn),
    MethodFn(TypeMethodFnDefn),
    AssociatedType(TypeAssociatedTypeDefn),
    AssociatedVal(TypeAssociatedValDefn),
    MemoizedField(TypeMemoizedFieldDefn),
}

impl TypeItemSynDefn {
    pub fn path(self, _db: &dyn SynDefnDb) -> AssociatedItemPath {
        todo!()
    }

    pub fn decl(self, db: &dyn SynDefnDb) -> TypeItemDecl {
        match self {
            TypeItemSynDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TypeItemSynDefn::MethodFn(defn) => defn.decl(db).into(),
            TypeItemSynDefn::AssociatedType(defn) => defn.decl(db).into(),
            TypeItemSynDefn::AssociatedVal(defn) => defn.decl(db).into(),
            TypeItemSynDefn::MemoizedField(defn) => defn.decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            TypeItemSynDefn::AssociatedFn(defn) => defn.expr_region(db).into(),
            TypeItemSynDefn::MethodFn(defn) => defn.expr_region(db).into(),
            TypeItemSynDefn::AssociatedType(defn) => defn.expr_region(db).into(),
            TypeItemSynDefn::AssociatedVal(defn) => defn.expr_region(db).into(),
            TypeItemSynDefn::MemoizedField(defn) => defn.expr_region(db).into(),
        }
    }
}

impl HasDefn for TypeItemPath {
    type Defn = TypeItemSynDefn;

    fn defn(self, db: &dyn SynDefnDb) -> DefnResult<Self::Defn> {
        ty_item_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_item_defn(db: &dyn SynDefnDb, path: TypeItemPath) -> DefnResult<TypeItemSynDefn> {
    let decl = path.decl(db)?;
    Ok(match decl {
        TypeItemDecl::AssociatedFn(decl) => TypeAssociatedFnDefn::new(db, path, decl)?.into(),
        TypeItemDecl::MethodFn(decl) => TypeMethodFnDefn::new(db, path, decl)?.into(),
        TypeItemDecl::AssociatedType(_) => todo!(),
        TypeItemDecl::AssociatedVal(_) => todo!(),
        TypeItemDecl::MemoizedField(decl) => TypeMemoizedFieldDefn::new(db, path, decl)?.into(),
    })
}
