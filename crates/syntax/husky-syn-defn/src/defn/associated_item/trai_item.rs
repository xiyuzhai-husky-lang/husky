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
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TraitItemSynNodeDefn {
    AssociatedFn(TraitAssociatedFnSynNodeDefn),
    MethodFn(TraitMethodFnSynNodeDefn),
    AssociatedType(TraitAssociatedTypeSynNodeDefn),
    AssociatedVal(TraitAssociatedValSynNodeDefn),
}

impl HasSynNodeDefn for TraitItemSynNodePath {
    type NodeDefn = TraitItemSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TraitItemSynDefn {
    AssociatedFn(TraitAssociatedFnSynDefn),
    MethodFn(TraitMethodFnSynDefn),
    AssociatedType(TraitAssociatedTypeSynDefn),
    AssociatedVal(TraitAssociatedValSynDefn),
}

impl TraitItemSynDefn {
    pub fn decl(self, _db: &dyn SynDefnDb) -> TraitItemDecl {
        todo!()
    }

    pub fn path(self, _db: &dyn SynDefnDb) -> AssociatedItemPath {
        todo!()
    }
    pub fn expr_region(self, _db: &dyn SynDefnDb) -> SynExprRegion {
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
    let decl = path.decl(db)?;
    Ok(match decl {
        TraitItemDecl::AssociatedFn(decl) => TraitAssociatedFnSynDefn::new(db, path, decl)?.into(),
        TraitItemDecl::MethodFn(decl) => TraitMethodFnSynDefn::new(db, path, decl)?.into(),
        TraitItemDecl::AssociatedType(_decl) => todo!(),
        TraitItemDecl::AssociatedVal(_decl) => todo!(),
    })
}
