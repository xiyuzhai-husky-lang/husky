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
    AssociatedFn(TraitAssociatedFnNodeDefn),
    MethodFn(TraitMethodFnNodeDefn),
    AssociatedType(TraitAssociatedTypeNodeDefn),
    AssociatedVal(TraitAssociatedValNodeDefn),
}

impl HasSynNodeDefn for TraitItemSynNodePath {
    type NodeDefn = TraitItemSynNodeDefn;

    fn node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TraitItemSynDefn {
    AssociatedFn(TraitAssociatedFnDefn),
    MethodFn(TraitMethodFnDefn),
    AssociatedType(TraitAssociatedTypeDefn),
    AssociatedVal(TraitAssociatedValDefn),
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

impl HasDefn for TraitItemPath {
    type Defn = TraitItemSynDefn;

    fn defn(self, db: &dyn SynDefnDb) -> DefnResult<Self::Defn> {
        trai_item_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_item_defn(
    db: &dyn SynDefnDb,
    path: TraitItemPath,
) -> DefnResult<TraitItemSynDefn> {
    let decl = path.decl(db)?;
    Ok(match decl {
        TraitItemDecl::AssociatedFn(decl) => TraitAssociatedFnDefn::new(db, path, decl)?.into(),
        TraitItemDecl::MethodFn(decl) => TraitMethodFnDefn::new(db, path, decl)?.into(),
        TraitItemDecl::AssociatedType(_decl) => todo!(),
        TraitItemDecl::AssociatedVal(_decl) => todo!(),
    })
}
