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
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TraitItemNodeDefn {
    AssociatedFn(TraitAssociatedFnNodeDefn),
    MethodFn(TraitMethodFnNodeDefn),
    AssociatedType(TraitAssociatedTypeNodeDefn),
    AssociatedVal(TraitAssociatedValNodeDefn),
}

impl HasNodeDefn for TraitItemNodePath {
    type NodeDefn = TraitItemNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TraitItemDefn {
    AssociatedFn(TraitAssociatedFnDefn),
    MethodFn(TraitMethodFnDefn),
    AssociatedType(TraitAssociatedTypeDefn),
    AssociatedVal(TraitAssociatedValDefn),
}

impl TraitItemDefn {
    pub fn decl(self, _db: &dyn DefnDb) -> TraitItemDecl {
        todo!()
    }

    pub fn path(self, _db: &dyn DefnDb) -> AssociatedItemPath {
        todo!()
    }
    pub fn expr_region(self, _db: &dyn DefnDb) -> ExprRegion {
        todo!()
    }
}

impl HasDefn for TraitItemPath {
    type Defn = TraitItemDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        trai_item_defn(db, self)
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_item_defn(db: &dyn DefnDb, path: TraitItemPath) -> DefnResult<TraitItemDefn> {
    let decl = path.decl(db)?;
    Ok(match decl {
        TraitItemDecl::AssociatedFn(decl) => TraitAssociatedFnDefn::new(db, path, decl)?.into(),
        TraitItemDecl::MethodFn(decl) => TraitMethodFnDefn::new(db, path, decl)?.into(),
        TraitItemDecl::AssociatedType(_decl) => todo!(),
        TraitItemDecl::AssociatedVal(_decl) => todo!(),
    })
}
