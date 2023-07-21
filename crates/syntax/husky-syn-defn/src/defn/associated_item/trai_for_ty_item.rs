mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemNodeDefn {
    AssociatedFn(TraitForTypeAssociatedFnNodeDefn),
    MethodFn(TraitForTypeMethodFnNodeDefn),
    AssociatedType(TraitForTypeAssociatedTypeNodeDefn),
    AssociatedVal(TraitForTypeAssociatedValNodeDefn),
    // todo: MemoizedField
}

impl TraitForTypeItemNodeDefn {
    pub fn node_path(self, db: &dyn DefnDb) -> TraitForTypeItemNodePath {
        match self {
            TraitForTypeItemNodeDefn::AssociatedFn(node_defn) => node_defn.node_path(db),
            TraitForTypeItemNodeDefn::MethodFn(node_defn) => node_defn.node_path(db),
            TraitForTypeItemNodeDefn::AssociatedType(node_defn) => node_defn.node_path(db),
            TraitForTypeItemNodeDefn::AssociatedVal(node_defn) => node_defn.node_path(db),
            // TraitForTypeItemNodeDefn::MemoizedField(node_defn) => node_defn.node_path(db),
        }
    }

    pub fn node_decl(self, db: &dyn DefnDb) -> TraitForTypeItemNodeDecl {
        match self {
            TraitForTypeItemNodeDefn::AssociatedFn(node_defn) => node_defn.node_decl(db).into(),
            TraitForTypeItemNodeDefn::MethodFn(node_defn) => node_defn.node_decl(db).into(),
            TraitForTypeItemNodeDefn::AssociatedType(node_defn) => node_defn.node_decl(db).into(),
            TraitForTypeItemNodeDefn::AssociatedVal(node_defn) => node_defn.node_decl(db).into(),
            // TraitForTypeItemNodeDefn::MemoizedField(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            TraitForTypeItemNodeDefn::AssociatedFn(node_defn) => node_defn.expr_region(db),
            TraitForTypeItemNodeDefn::MethodFn(node_defn) => node_defn.expr_region(db),
            TraitForTypeItemNodeDefn::AssociatedType(node_defn) => node_defn.expr_region(db),
            TraitForTypeItemNodeDefn::AssociatedVal(node_defn) => node_defn.expr_region(db),
        }
    }
}

impl HasNodeDefn for TraitForTypeItemNodePath {
    type NodeDefn = TraitForTypeItemNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        trai_for_ty_item_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_for_ty_item_node_defn(
    db: &dyn DefnDb,
    node_path: TraitForTypeItemNodePath,
) -> TraitForTypeItemNodeDefn {
    match node_path.node_decl(db) {
        TraitForTypeItemNodeDecl::AssociatedFn(_) => todo!(),
        TraitForTypeItemNodeDecl::MethodFn(node_decl) => {
            TraitForTypeMethodFnNodeDefn::new(db, node_path, node_decl).into()
        }
        TraitForTypeItemNodeDecl::AssociatedType(node_decl) => {
            TraitForTypeAssociatedTypeNodeDefn::new(db, node_path, node_decl).into()
        }
        TraitForTypeItemNodeDecl::AssociatedVal(_) => todo!(),
        // TraitForTypeItemNodeDecl::MemoizedField(_) => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemDefn {
    AssociatedFn(TraitForTypeAssociatedFnDefn),
    MethodFn(TraitForTypeMethodFnDefn),
    AssociatedType(TraitForTypeAssociatedTypeDefn),
    AssociatedVal(TraitForTypeAssociatedValDefn),
}

impl TraitForTypeItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TraitForTypeItemDecl {
        match self {
            TraitForTypeItemDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TraitForTypeItemDefn::MethodFn(defn) => defn.decl(db).into(),
            TraitForTypeItemDefn::AssociatedType(defn) => defn.decl(db).into(),
            TraitForTypeItemDefn::AssociatedVal(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, _db: &dyn DefnDb) -> TraitForTypeItemPath {
        todo!()
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            TraitForTypeItemDefn::AssociatedFn(defn) => defn.expr_region(db),
            TraitForTypeItemDefn::MethodFn(defn) => defn.expr_region(db),
            TraitForTypeItemDefn::AssociatedType(defn) => defn.expr_region(db),
            TraitForTypeItemDefn::AssociatedVal(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for TraitForTypeItemPath {
    type Defn = TraitForTypeItemDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        trai_for_ty_item_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_for_ty_item_defn(
    db: &dyn DefnDb,
    path: TraitForTypeItemPath,
) -> DefnResult<TraitForTypeItemDefn> {
    Ok(match path.decl(db)? {
        TraitForTypeItemDecl::AssociatedFn(_) => todo!(),
        TraitForTypeItemDecl::MethodFn(decl) => {
            TraitForTypeMethodFnDefn::new(db, path, decl).into()
        }
        TraitForTypeItemDecl::AssociatedType(decl) => {
            TraitForTypeAssociatedTypeDefn::new(db, path, decl).into()
        }
        TraitForTypeItemDecl::AssociatedVal(decl) => {
            TraitForTypeAssociatedValDefn::new(db, path, decl)?.into()
        }
    })
}
