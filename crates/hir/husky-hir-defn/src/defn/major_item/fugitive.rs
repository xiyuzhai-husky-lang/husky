mod r#fn;
mod gn;
mod type_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::type_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum FugitiveHirDefn {
    FunctionFn(FunctionFnHirDefn),
    // Function(FunctionDefn),
    Val(ValHirDefn),
    FunctionGn(FunctionGnHirDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveHirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> FugitivePath {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.path(db),
            FugitiveHirDefn::Val(hir_defn) => hir_defn.path(db),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &dyn HirDefnDb) -> FugitiveHirDecl {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.hir_decl(db).into(),
            FugitiveHirDefn::Val(hir_defn) => hir_defn.hir_decl(db).into(),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => {
                hir_defn.hir_eager_expr_region(db).map(Into::into)
            }
            FugitiveHirDefn::Val(hir_defn) => hir_defn.hir_expr_region(db),
            FugitiveHirDefn::FunctionGn(hir_defn) => {
                hir_defn.hir_lazy_expr_region(db).map(Into::into)
            }
        }
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.dependencies(db),
            FugitiveHirDefn::Val(hir_defn) => hir_defn.dependencies(db),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.version_stamp(db),
            FugitiveHirDefn::Val(hir_defn) => hir_defn.version_stamp(db),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for FugitivePath {
    type HirDefn = FugitiveHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        fugitive_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn fugitive_hir_defn(db: &dyn HirDefnDb, path: FugitivePath) -> Option<FugitiveHirDefn> {
    match path.hir_decl(db)? {
        FugitiveHirDecl::FunctionFn(hir_decl) => {
            Some(FunctionFnHirDefn::new(db, path, hir_decl).into())
        }
        FugitiveHirDecl::Val(hir_decl) => Some(ValHirDefn::new(db, path, hir_decl).into()),
        FugitiveHirDecl::FunctionGn(hir_decl) => {
            Some(FunctionGnHirDefn::new(db, path, hir_decl).into())
        }
        FugitiveHirDecl::TypeAlias(_) => todo!(),
    }
}
