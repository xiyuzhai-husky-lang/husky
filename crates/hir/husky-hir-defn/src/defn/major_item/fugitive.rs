mod r#fn;
mod gn;
mod type_alias;
mod val;

use husky_hir_decl::decl::FugitiveHirDecl;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::type_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum FugitiveHirDefn {
    FunctionFn(FunctionFnHirDefn),
    // Function(FunctionDefn),
    Ki(ValHirDefn),
    FunctionGn(FunctionGnHirDefn),
    TypeAlias(TypeAliasHirDefn),
}

impl From<FugitiveHirDefn> for HirDefn {
    fn from(hir_defn: FugitiveHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl FugitiveHirDefn {
    pub fn path(self, db: &::salsa::Db) -> FugitivePath {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.path(db),
            FugitiveHirDefn::Ki(hir_defn) => hir_defn.path(db),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.path(db),
            FugitiveHirDefn::TypeAlias(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> FugitiveHirDecl {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.hir_decl(db).into(),
            FugitiveHirDefn::Ki(hir_defn) => hir_defn.hir_decl(db).into(),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.hir_decl(db).into(),
            FugitiveHirDefn::TypeAlias(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => {
                hir_defn.hir_eager_expr_region(db).map(Into::into)
            }
            FugitiveHirDefn::Ki(hir_defn) => hir_defn.hir_expr_region(db),
            FugitiveHirDefn::FunctionGn(hir_defn) => {
                hir_defn.hir_lazy_expr_region(db).map(Into::into)
            }
            FugitiveHirDefn::TypeAlias(hir_defn) => {
                hir_defn.hir_eager_expr_region(db).map(Into::into)
            }
        }
    }

    pub fn hir_expr_body_and_region(self, db: &::salsa::Db) -> Option<(HirExprIdx, HirExprRegion)> {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn
                .eager_body_with_hir_eager_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            FugitiveHirDefn::Ki(hir_defn) => hir_defn
                .hir_expr_body_and_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn
                .lazy_body_with_hir_lazy_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            FugitiveHirDefn::TypeAlias(hir_defn) => None,
        }
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.dependencies(db),
            FugitiveHirDefn::Ki(hir_defn) => hir_defn.dependencies(db),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.dependencies(db),
            FugitiveHirDefn::TypeAlias(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.version_stamp(db),
            FugitiveHirDefn::Ki(hir_defn) => hir_defn.version_stamp(db),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.version_stamp(db),
            FugitiveHirDefn::TypeAlias(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for FugitivePath {
    type HirDefn = FugitiveHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        fugitive_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn fugitive_hir_defn(db: &::salsa::Db, path: FugitivePath) -> Option<FugitiveHirDefn> {
    match path.hir_decl(db)? {
        FugitiveHirDecl::FunctionFn(hir_decl) => {
            Some(FunctionFnHirDefn::new(db, path, hir_decl).into())
        }
        FugitiveHirDecl::Ki(hir_decl) => Some(ValHirDefn::new(db, path, hir_decl).into()),
        FugitiveHirDecl::FunctionGn(hir_decl) => {
            Some(FunctionGnHirDefn::new(db, path, hir_decl).into())
        }
        FugitiveHirDecl::TypeAlias(_) => todo!(),
    }
}
