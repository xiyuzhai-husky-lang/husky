pub mod r#const;
pub mod function_ritchie;
pub mod r#static;
pub mod ty_alias;
pub mod val;

use self::function_ritchie::*;
use self::r#const::*;
use self::r#static::*;
use self::ty_alias::*;
use self::val::*;
use super::*;
use husky_entity_path::path::major_item::form::MajorFormPath;
use husky_hir_decl::decl::MajorFormHirDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorFormHirDefn {
    Ritchie(MajorFunctionRitchieHirDefn),
    Val(MajorValHirDefn),
    Const(MajorConstHirDefn),
    Static(MajorStaticHirDefn),
    TypeAlias(TypeAliasHirDefn),
}

impl From<MajorFormHirDefn> for HirDefn {
    fn from(hir_defn: MajorFormHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl MajorFormHirDefn {
    pub fn path(self, db: &::salsa::Db) -> MajorFormPath {
        match self {
            MajorFormHirDefn::Ritchie(slf) => slf.path(db),
            MajorFormHirDefn::Val(slf) => slf.path(db),
            MajorFormHirDefn::TypeAlias(slf) => slf.path(db),
            MajorFormHirDefn::Const(slf) => slf.path(db),
            MajorFormHirDefn::Static(slf) => slf.path(db),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> MajorFormHirDecl {
        match self {
            MajorFormHirDefn::Ritchie(slf) => slf.hir_decl(db).into(),
            MajorFormHirDefn::Val(slf) => slf.hir_decl(db).into(),
            MajorFormHirDefn::TypeAlias(slf) => slf.hir_decl(db).into(),
            MajorFormHirDefn::Const(slf) => slf.hir_decl(db).into(),
            MajorFormHirDefn::Static(slf) => slf.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            MajorFormHirDefn::Ritchie(slf) => slf.hir_expr_region(db),
            MajorFormHirDefn::Val(slf) => slf.hir_expr_region(db),
            MajorFormHirDefn::TypeAlias(slf) => slf.hir_eager_expr_region(db).map(Into::into),
            MajorFormHirDefn::Const(slf) => slf.hir_expr_region(db),
            MajorFormHirDefn::Static(slf) => slf.hir_expr_region(db),
        }
    }

    pub fn hir_expr_body_and_region(self, db: &::salsa::Db) -> Option<(HirExprIdx, HirExprRegion)> {
        match self {
            MajorFormHirDefn::Ritchie(slf) => slf
                .body_with_hir_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            MajorFormHirDefn::Val(slf) => slf
                .hir_expr_body_and_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            MajorFormHirDefn::TypeAlias(slf) => None,
            MajorFormHirDefn::Const(slf) => slf.hir_expr_body_and_region(db),
            MajorFormHirDefn::Static(slf) => slf.hir_expr_body_and_region(db),
        }
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            MajorFormHirDefn::Ritchie(slf) => slf.dependencies(db),
            MajorFormHirDefn::Val(slf) => slf.dependencies(db),
            MajorFormHirDefn::TypeAlias(slf) => slf.dependencies(db),
            MajorFormHirDefn::Const(slf) => slf.dependencies(db),
            MajorFormHirDefn::Static(slf) => slf.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            MajorFormHirDefn::Ritchie(slf) => slf.version_stamp(db),
            MajorFormHirDefn::Val(slf) => slf.version_stamp(db),
            MajorFormHirDefn::TypeAlias(slf) => slf.version_stamp(db),
            MajorFormHirDefn::Const(slf) => slf.version_stamp(db),
            MajorFormHirDefn::Static(slf) => slf.version_stamp(db),
        }
    }
}

impl HasHirDefn for MajorFormPath {
    type HirDefn = MajorFormHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        form_hir_defn(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn form_hir_defn(db: &::salsa::Db, path: MajorFormPath) -> Option<MajorFormHirDefn> {
    match path.hir_decl(db)? {
        MajorFormHirDecl::Ritchie(hir_decl) => {
            Some(MajorFunctionRitchieHirDefn::new(db, path, hir_decl).into())
        }
        MajorFormHirDecl::Val(hir_decl) => Some(MajorValHirDefn::new(db, path, hir_decl).into()),
        MajorFormHirDecl::TypeAlias(_) => todo!(),
        MajorFormHirDecl::Const(hir_decl) => {
            Some(MajorConstHirDefn::new(db, path, hir_decl).into())
        }
        MajorFormHirDecl::Static(hir_decl) => {
            Some(MajorStaticHirDefn::new(db, path, hir_decl).into())
        }
    }
}
