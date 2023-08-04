mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;
use husky_entity_path::AssociatedItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TraitItemSynNodeDefn {
    AssociatedFn(TraitAssociatedFnSynNodeDefn),
    MethodFn(TraitMethodFnSynNodeDefn),
    AssociatedType(TraitAssociatedTypeSynNodeDefn),
    AssociatedVal(TraitAssociatedValSynNodeDefn),
}

impl TraitItemSynNodeDefn {
    pub fn syn_expr_region(self, db: &dyn SynDefnDb) -> SynExprRegion {
        match self {
            TraitItemSynNodeDefn::AssociatedFn(syn_node_defn) => syn_node_defn.syn_expr_region(db),
            TraitItemSynNodeDefn::MethodFn(syn_node_defn) => syn_node_defn.syn_expr_region(db),
            TraitItemSynNodeDefn::AssociatedType(syn_node_defn) => {
                syn_node_defn.syn_expr_region(db)
            }
            TraitItemSynNodeDefn::AssociatedVal(syn_node_defn) => syn_node_defn.syn_expr_region(db),
        }
    }
}

impl HasSynNodeDefn for TraitItemSynNodePath {
    type SynNodeDefn = TraitItemSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        trai_item_syn_node_defn(db, self)
    }
}

impl TraitItemSynNodeDefn {
    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> TraitItemSynNodeDecl {
        match self {
            TraitItemSynNodeDefn::AssociatedFn(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TraitItemSynNodeDefn::MethodFn(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TraitItemSynNodeDefn::AssociatedType(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TraitItemSynNodeDefn::AssociatedVal(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
        }
    }
}

#[salsa::tracked(jar = SynDefnJar)]
fn trai_item_syn_node_defn(
    db: &dyn SynDefnDb,
    syn_node_path: TraitItemSynNodePath,
) -> TraitItemSynNodeDefn {
    match syn_node_path.syn_node_decl(db) {
        TraitItemSynNodeDecl::AssociatedFn(_) => todo!(),
        TraitItemSynNodeDecl::MethodFn(syn_node_decl) => {
            TraitMethodFnSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TraitItemSynNodeDecl::AssociatedType(syn_node_decl) => {
            TraitAssociatedTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TraitItemSynNodeDecl::AssociatedVal(_) => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TraitItemSynDefn {
    AssociatedFn(TraitAssociatedFnSynDefn),
    MethodFn(TraitMethodFnSynDefn),
    AssociatedType(TraitAssociatedTypeSynDefn),
    AssociatedVal(TraitAssociatedValSynDefn),
}

impl TraitItemSynDefn {
    pub fn decl(self, _db: &dyn SynDefnDb) -> TraitItemSynDecl {
        todo!()
    }

    pub fn path(self, _db: &dyn SynDefnDb) -> AssociatedItemPath {
        todo!()
    }

    pub fn syn_expr_region(self, _db: &dyn SynDefnDb) -> SynExprRegion {
        todo!()
    }
}

impl HasSynDefn for TraitItemPath {
    type SynDefn = TraitItemSynDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        trai_item_syn_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_item_syn_defn(
    db: &dyn SynDefnDb,
    path: TraitItemPath,
) -> SynDefnResult<TraitItemSynDefn> {
    let decl = path.syn_decl(db)?;
    Ok(match decl {
        TraitItemSynDecl::AssociatedFn(decl) => {
            TraitAssociatedFnSynDefn::new(db, path, decl)?.into()
        }
        TraitItemSynDecl::MethodFn(decl) => TraitMethodFnSynDefn::new(db, path, decl)?.into(),
        TraitItemSynDecl::AssociatedType(_decl) => todo!(),
        TraitItemSynDecl::AssociatedVal(_decl) => todo!(),
    })
}
