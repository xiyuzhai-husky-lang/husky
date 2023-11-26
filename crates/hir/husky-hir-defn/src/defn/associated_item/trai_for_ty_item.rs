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
#[salsa::debug_with_db(db = HirDefnDb, jar = HirDefnJar)]
#[enum_class::from_variants]
pub enum TraitForTypeItemHirDefn {
    AssociatedFn(TraitForTypeAssociatedFnHirDefn),
    MethodFn(TraitForTypeMethodFnHirDefn),
    AssociatedType(TraitForTypeAssociatedTypeHirDefn),
    AssociatedVal(TraitForTypeAssociatedValHirDefn),
}

impl From<TraitForTypeItemHirDefn> for HirDefn {
    fn from(hir_defn: TraitForTypeItemHirDefn) -> Self {
        HirDefn::AssociatedItem(hir_defn.into())
    }
}

impl TraitForTypeItemHirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.path(db),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.path(db),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.path(db),
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &dyn HirDefnDb) -> TraitForTypeItemHirDecl {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, _db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        todo!()
        // match self {
        //     TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => {
        //         Some(hir_defn.hir_expr_region(db).into())
        //     }
        //     TraitForTypeItemHirDefn::MethodFn(hir_defn) => Some(hir_defn.hir_expr_region(db).into()),
        //     TraitForTypeItemHirDefn::AssociatedType(hir_defn) => {
        //         hir_defn.hir_expr_region(db).into()
        //     }
        //     TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.hir_expr_region(db).into(),
        // }
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.dependencies(db),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.dependencies(db),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.dependencies(db),
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.version_stamp(db),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.version_stamp(db),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.version_stamp(db),
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for TraitForTypeItemPath {
    type HirDefn = TraitForTypeItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        trai_for_ty_item_hir_defn(db, self)
    }
}

// #[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_for_ty_item_hir_defn(
    db: &dyn HirDefnDb,
    path: TraitForTypeItemPath,
) -> Option<TraitForTypeItemHirDefn> {
    match path.hir_decl(db)? {
        TraitForTypeItemHirDecl::AssociatedFn(_) => todo!(),
        TraitForTypeItemHirDecl::MethodFn(hir_decl) => {
            Some(TraitForTypeMethodFnHirDefn::new(db, path, hir_decl).into())
        }
        TraitForTypeItemHirDecl::AssociatedType(hir_decl) => {
            Some(TraitForTypeAssociatedTypeHirDefn::new(db, path, hir_decl).into())
        }
        TraitForTypeItemHirDecl::AssociatedVal(hir_decl) => {
            Some(TraitForTypeAssociatedValHirDefn::new(db, path, hir_decl).into())
        }
    }
}
