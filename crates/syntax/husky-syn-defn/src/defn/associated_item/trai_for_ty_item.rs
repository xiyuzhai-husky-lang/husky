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
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemSynNodeDefn {
    AssociatedFn(TraitForTypeAssociatedFnNodeDefn),
    MethodFn(TraitForTypeMethodFnNodeDefn),
    AssociatedType(TraitForTypeAssociatedTypeNodeDefn),
    AssociatedVal(TraitForTypeAssociatedValNodeDefn),
    // todo: MemoizedField
}

impl TraitForTypeItemSynNodeDefn {
    pub fn syn_node_path(self, db: &dyn SynDefnDb) -> TraitForTypeItemSynNodePath {
        match self {
            TraitForTypeItemSynNodeDefn::AssociatedFn(node_defn) => node_defn.syn_node_path(db),
            TraitForTypeItemSynNodeDefn::MethodFn(node_defn) => node_defn.syn_node_path(db),
            TraitForTypeItemSynNodeDefn::AssociatedType(node_defn) => node_defn.syn_node_path(db),
            TraitForTypeItemSynNodeDefn::AssociatedVal(node_defn) => node_defn.syn_node_path(db),
            // TraitForTypeItemNodeDefn::MemoizedField(node_defn) => node_defn.syn_node_path(db),
        }
    }

    pub fn node_decl(self, db: &dyn SynDefnDb) -> TraitForTypeItemNodeDecl {
        match self {
            TraitForTypeItemSynNodeDefn::AssociatedFn(node_defn) => node_defn.node_decl(db).into(),
            TraitForTypeItemSynNodeDefn::MethodFn(node_defn) => node_defn.node_decl(db).into(),
            TraitForTypeItemSynNodeDefn::AssociatedType(node_defn) => {
                node_defn.node_decl(db).into()
            }
            TraitForTypeItemSynNodeDefn::AssociatedVal(node_defn) => node_defn.node_decl(db).into(),
            // TraitForTypeItemNodeDefn::MemoizedField(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> SynExprRegion {
        match self {
            TraitForTypeItemSynNodeDefn::AssociatedFn(node_defn) => node_defn.expr_region(db),
            TraitForTypeItemSynNodeDefn::MethodFn(node_defn) => node_defn.expr_region(db),
            TraitForTypeItemSynNodeDefn::AssociatedType(node_defn) => node_defn.expr_region(db),
            TraitForTypeItemSynNodeDefn::AssociatedVal(node_defn) => node_defn.expr_region(db),
        }
    }
}

impl HasSynNodeDefn for TraitForTypeItemSynNodePath {
    type NodeDefn = TraitForTypeItemSynNodeDefn;

    fn node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        trai_for_ty_item_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_for_ty_item_node_defn(
    db: &dyn SynDefnDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> TraitForTypeItemSynNodeDefn {
    match syn_node_path.node_decl(db) {
        TraitForTypeItemNodeDecl::AssociatedFn(_) => todo!(),
        TraitForTypeItemNodeDecl::MethodFn(node_decl) => {
            TraitForTypeMethodFnNodeDefn::new(db, syn_node_path, node_decl).into()
        }
        TraitForTypeItemNodeDecl::AssociatedType(node_decl) => {
            TraitForTypeAssociatedTypeNodeDefn::new(db, syn_node_path, node_decl).into()
        }
        TraitForTypeItemNodeDecl::AssociatedVal(_) => todo!(),
        // TraitForTypeItemNodeDecl::MemoizedField(_) => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemSynDefn {
    AssociatedFn(TraitForTypeAssociatedFnDefn),
    MethodFn(TraitForTypeMethodFnDefn),
    AssociatedType(TraitForTypeAssociatedTypeDefn),
    AssociatedVal(TraitForTypeAssociatedValDefn),
}

impl TraitForTypeItemSynDefn {
    pub fn decl(self, db: &dyn SynDefnDb) -> TraitForTypeItemDecl {
        match self {
            TraitForTypeItemSynDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TraitForTypeItemSynDefn::MethodFn(defn) => defn.decl(db).into(),
            TraitForTypeItemSynDefn::AssociatedType(defn) => defn.decl(db).into(),
            TraitForTypeItemSynDefn::AssociatedVal(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, _db: &dyn SynDefnDb) -> TraitForTypeItemPath {
        todo!()
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> SynExprRegion {
        match self {
            TraitForTypeItemSynDefn::AssociatedFn(defn) => defn.expr_region(db),
            TraitForTypeItemSynDefn::MethodFn(defn) => defn.expr_region(db),
            TraitForTypeItemSynDefn::AssociatedType(defn) => defn.expr_region(db),
            TraitForTypeItemSynDefn::AssociatedVal(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for TraitForTypeItemPath {
    type Defn = TraitForTypeItemSynDefn;

    fn defn(self, db: &dyn SynDefnDb) -> DefnResult<Self::Defn> {
        trai_for_ty_item_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_for_ty_item_defn(
    db: &dyn SynDefnDb,
    path: TraitForTypeItemPath,
) -> DefnResult<TraitForTypeItemSynDefn> {
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
