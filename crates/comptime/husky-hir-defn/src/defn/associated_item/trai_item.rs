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
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TraitItemHirDefn {
    AssociatedFn(TraitAssociatedFnHirDefn),
    MethodFn(TraitMethodFnHirDefn),
    AssociatedType(TraitAssociatedTypeHirDefn),
    AssociatedVal(TraitAssociatedValHirDefn),
}

impl TraitItemHirDefn {
    pub fn decl(self, _db: &dyn HirDefnDb) -> TraitItemDecl {
        todo!()
    }

    pub fn path(self, _db: &dyn HirDefnDb) -> AssociatedItemPath {
        todo!()
    }
    pub fn expr_region(self, _db: &dyn HirDefnDb) -> HirExprRegion {
        todo!()
    }
}

impl HasHirDefn for TraitItemPath {
    type HirDefn = TraitItemHirDefn;

    fn syn_defn(self, db: &dyn HirDefnDb) -> HirDefnResult<Self::HirDefn> {
        trai_item_syn_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_item_syn_defn(
    db: &dyn HirDefnDb,
    path: TraitItemPath,
) -> HirDefnResult<TraitItemHirDefn> {
    let decl = path.decl(db)?;
    Ok(match decl {
        TraitItemDecl::AssociatedFn(decl) => TraitAssociatedFnHirDefn::new(db, path, decl)?.into(),
        TraitItemDecl::MethodFn(decl) => TraitMethodFnHirDefn::new(db, path, decl)?.into(),
        TraitItemDecl::AssociatedType(_decl) => todo!(),
        TraitItemDecl::AssociatedVal(_decl) => todo!(),
    })
}
