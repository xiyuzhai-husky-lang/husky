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
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum TraitForTypeItemSynNodeDefn {
    AssociatedFn(TraitForTypeAssociatedFnSynNodeDefn),
    MethodFn(TraitForTypeMethodFnSynNodeDefn),
    AssociatedType(TraitForTypeAssociatedTypeSynNodeDefn),
    AssociatedVal(TraitForTypeAssociatedValSynNodeDefn),
    // todo: MemoizedField
}

impl TraitForTypeItemSynNodeDefn {
    pub fn syn_node_path(self, db: &dyn SynDefnDb) -> TraitForTypeItemSynNodePath {
        match self {
            TraitForTypeItemSynNodeDefn::AssociatedFn(syn_node_defn) => {
                syn_node_defn.syn_node_path(db)
            }
            TraitForTypeItemSynNodeDefn::MethodFn(syn_node_defn) => syn_node_defn.syn_node_path(db),
            TraitForTypeItemSynNodeDefn::AssociatedType(syn_node_defn) => {
                syn_node_defn.syn_node_path(db)
            }
            TraitForTypeItemSynNodeDefn::AssociatedVal(syn_node_defn) => {
                syn_node_defn.syn_node_path(db)
            } // TraitForTypeItemNodeDefn::MemoizedField(syn_node_defn) => syn_node_defn.syn_node_path(db),
        }
    }

    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> TraitForTypeItemSynNodeDecl {
        match self {
            TraitForTypeItemSynNodeDefn::AssociatedFn(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TraitForTypeItemSynNodeDefn::MethodFn(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TraitForTypeItemSynNodeDefn::AssociatedType(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TraitForTypeItemSynNodeDefn::AssociatedVal(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            } // TraitForTypeItemNodeDefn::MemoizedField(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &dyn SynDefnDb,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            TraitForTypeItemSynNodeDefn::AssociatedFn(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
            TraitForTypeItemSynNodeDefn::MethodFn(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
            TraitForTypeItemSynNodeDefn::AssociatedType(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
            TraitForTypeItemSynNodeDefn::AssociatedVal(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
        }
    }
}

impl HasSynNodeDefn for TraitForTypeItemSynNodePath {
    type SynNodeDefn = TraitForTypeItemSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        trai_for_ty_item_syn_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_for_ty_item_syn_node_defn(
    db: &dyn SynDefnDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> TraitForTypeItemSynNodeDefn {
    match syn_node_path.syn_node_decl(db) {
        TraitForTypeItemSynNodeDecl::AssociatedFn(_) => todo!(),
        TraitForTypeItemSynNodeDecl::MethodFn(syn_node_decl) => {
            TraitForTypeMethodFnSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TraitForTypeItemSynNodeDecl::AssociatedType(syn_node_decl) => {
            TraitForTypeAssociatedTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TraitForTypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
        // TraitForTypeItemNodeDecl::MemoizedField(_) => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum TraitForTypeItemSynDefn {
    AssociatedFn(TraitForTypeAssociatedFnSynDefn),
    MethodFn(TraitForTypeMethodFnSynDefn),
    AssociatedType(TraitForTypeAssociatedTypeSynDefn),
    AssociatedVal(TraitForTypeAssociatedValSynDefn),
}

impl TraitForTypeItemSynDefn {
    pub fn decl(self, db: &dyn SynDefnDb) -> TraitForTypeItemSynDecl {
        match self {
            TraitForTypeItemSynDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TraitForTypeItemSynDefn::MethodFn(defn) => defn.decl(db).into(),
            TraitForTypeItemSynDefn::AssociatedType(defn) => defn.decl(db).into(),
            TraitForTypeItemSynDefn::AssociatedVal(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn SynDefnDb) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemSynDefn::AssociatedFn(defn) => defn.path(db),
            TraitForTypeItemSynDefn::MethodFn(defn) => defn.path(db),
            TraitForTypeItemSynDefn::AssociatedType(defn) => defn.path(db),
            TraitForTypeItemSynDefn::AssociatedVal(defn) => defn.path(db),
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &dyn SynDefnDb,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            TraitForTypeItemSynDefn::AssociatedFn(defn) => defn.body_with_syn_expr_region(db),
            TraitForTypeItemSynDefn::MethodFn(defn) => defn.body_with_syn_expr_region(db),
            TraitForTypeItemSynDefn::AssociatedType(defn) => defn.body_with_syn_expr_region(db),
            TraitForTypeItemSynDefn::AssociatedVal(defn) => defn.body_with_syn_expr_region(db),
        }
    }
}

impl HasSynDefn for TraitForTypeItemPath {
    type SynDefn = TraitForTypeItemSynDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        trai_for_ty_item_syn_defn(db, self)
    }
}

// #[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_for_ty_item_syn_defn(
    db: &dyn SynDefnDb,
    path: TraitForTypeItemPath,
) -> SynDefnResult<TraitForTypeItemSynDefn> {
    Ok(match path.syn_decl(db)? {
        TraitForTypeItemSynDecl::AssociatedFn(_) => todo!(),
        TraitForTypeItemSynDecl::MethodFn(decl) => {
            TraitForTypeMethodFnSynDefn::new(db, path, decl).into()
        }
        TraitForTypeItemSynDecl::AssociatedType(decl) => {
            TraitForTypeAssociatedTypeSynDefn::new(db, path, decl).into()
        }
        TraitForTypeItemSynDecl::AssociatedVal(decl) => {
            TraitForTypeAssociatedValSynDefn::new(db, path, decl)?.into()
        }
    })
}
