mod function_ritchie;
mod type_alias;
mod val;

use husky_hir_decl::decl::MajorFormHirDecl;

pub use self::function_ritchie::*;
pub use self::type_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorFormHirDefn {
    Ritchie(MajorFunctionRitchieHirDefn),
    Val(ValHirDefn),
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
            MajorFormHirDefn::Ritchie(hir_defn) => hir_defn.path(db),
            MajorFormHirDefn::Val(hir_defn) => hir_defn.path(db),
            MajorFormHirDefn::TypeAlias(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> MajorFormHirDecl {
        match self {
            MajorFormHirDefn::Ritchie(hir_defn) => hir_defn.hir_decl(db).into(),
            MajorFormHirDefn::Val(hir_defn) => hir_defn.hir_decl(db).into(),
            MajorFormHirDefn::TypeAlias(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            MajorFormHirDefn::Ritchie(hir_defn) => hir_defn.hir_expr_region(db),
            MajorFormHirDefn::Val(hir_defn) => hir_defn.hir_expr_region(db),
            MajorFormHirDefn::TypeAlias(hir_defn) => {
                hir_defn.hir_eager_expr_region(db).map(Into::into)
            }
        }
    }

    pub fn hir_expr_body_and_region(self, db: &::salsa::Db) -> Option<(HirExprIdx, HirExprRegion)> {
        match self {
            MajorFormHirDefn::Ritchie(hir_defn) => hir_defn
                .body_with_hir_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            MajorFormHirDefn::Val(hir_defn) => hir_defn
                .hir_expr_body_and_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            MajorFormHirDefn::TypeAlias(hir_defn) => None,
        }
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            MajorFormHirDefn::Ritchie(hir_defn) => hir_defn.dependencies(db),
            MajorFormHirDefn::Val(hir_defn) => hir_defn.dependencies(db),
            MajorFormHirDefn::TypeAlias(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            MajorFormHirDefn::Ritchie(hir_defn) => hir_defn.version_stamp(db),
            MajorFormHirDefn::Val(hir_defn) => hir_defn.version_stamp(db),
            MajorFormHirDefn::TypeAlias(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for MajorFormPath {
    type HirDefn = MajorFormHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        form_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn form_hir_defn(db: &::salsa::Db, path: MajorFormPath) -> Option<MajorFormHirDefn> {
    match path.hir_decl(db)? {
        MajorFormHirDecl::Ritchie(hir_decl) => {
            Some(MajorFunctionRitchieHirDefn::new(db, path, hir_decl).into())
        }
        MajorFormHirDecl::Val(hir_decl) => Some(ValHirDefn::new(db, path, hir_decl).into()),
        MajorFormHirDecl::TypeAlias(_) => todo!(),
        MajorFormHirDecl::Const(_) => todo!(),
    }
}
