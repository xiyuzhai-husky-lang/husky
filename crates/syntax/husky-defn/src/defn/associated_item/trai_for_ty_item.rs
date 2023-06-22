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

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_item_defn(
    db: &dyn DefnDb,
    path: TraitForTypeItemPath,
) -> DefnResult<TraitForTypeItemDefn> {
    Ok(match path.decl(db)? {
        TraitForTypeItemDecl::AssociatedFn(_) => todo!(),
        TraitForTypeItemDecl::MethodFn(decl) => {
            TraitForTypeMethodFnDefn::new(db, path, decl)?.into()
        }
        TraitForTypeItemDecl::AssociatedType(decl) => {
            TraitForTypeAssociatedTypeDefn::new(db, path, decl)?.into()
        }
        TraitForTypeItemDecl::AssociatedVal(decl) => {
            TraitForTypeAssociatedValDefn::new(db, path, decl)?.into()
        }
    })
}
