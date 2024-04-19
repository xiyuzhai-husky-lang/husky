mod gn;
mod ritchie;
mod type_alias;
mod val;

use husky_hir_decl::decl::MajorFormHirDecl;

pub use self::gn::*;
pub use self::ritchie::*;
pub use self::type_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum FormHirDefn {
    FunctionFn(MajorRitchieHirDefn),
    // Function(FunctionDefn),
    Ki(ValHirDefn),
    FunctionGn(FunctionGnHirDefn),
    TypeAlias(TypeAliasHirDefn),
}

impl From<FormHirDefn> for HirDefn {
    fn from(hir_defn: FormHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl FormHirDefn {
    pub fn path(self, db: &::salsa::Db) -> MajorFormPath {
        match self {
            FormHirDefn::FunctionFn(hir_defn) => hir_defn.path(db),
            FormHirDefn::Ki(hir_defn) => hir_defn.path(db),
            FormHirDefn::FunctionGn(hir_defn) => hir_defn.path(db),
            FormHirDefn::TypeAlias(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> MajorFormHirDecl {
        match self {
            FormHirDefn::FunctionFn(hir_defn) => hir_defn.hir_decl(db).into(),
            FormHirDefn::Ki(hir_defn) => hir_defn.hir_decl(db).into(),
            FormHirDefn::FunctionGn(hir_defn) => hir_defn.hir_decl(db).into(),
            FormHirDefn::TypeAlias(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            FormHirDefn::FunctionFn(hir_defn) => hir_defn.hir_eager_expr_region(db).map(Into::into),
            FormHirDefn::Ki(hir_defn) => hir_defn.hir_expr_region(db),
            FormHirDefn::FunctionGn(hir_defn) => hir_defn.hir_lazy_expr_region(db).map(Into::into),
            FormHirDefn::TypeAlias(hir_defn) => hir_defn.hir_eager_expr_region(db).map(Into::into),
        }
    }

    pub fn hir_expr_body_and_region(self, db: &::salsa::Db) -> Option<(HirExprIdx, HirExprRegion)> {
        match self {
            FormHirDefn::FunctionFn(hir_defn) => hir_defn
                .eager_body_with_hir_eager_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            FormHirDefn::Ki(hir_defn) => hir_defn
                .hir_expr_body_and_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            FormHirDefn::FunctionGn(hir_defn) => hir_defn
                .lazy_body_with_hir_lazy_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            FormHirDefn::TypeAlias(hir_defn) => None,
        }
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            FormHirDefn::FunctionFn(hir_defn) => hir_defn.dependencies(db),
            FormHirDefn::Ki(hir_defn) => hir_defn.dependencies(db),
            FormHirDefn::FunctionGn(hir_defn) => hir_defn.dependencies(db),
            FormHirDefn::TypeAlias(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            FormHirDefn::FunctionFn(hir_defn) => hir_defn.version_stamp(db),
            FormHirDefn::Ki(hir_defn) => hir_defn.version_stamp(db),
            FormHirDefn::FunctionGn(hir_defn) => hir_defn.version_stamp(db),
            FormHirDefn::TypeAlias(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for MajorFormPath {
    type HirDefn = FormHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        form_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn form_hir_defn(db: &::salsa::Db, path: MajorFormPath) -> Option<FormHirDefn> {
    match path.hir_decl(db)? {
        MajorFormHirDecl::Ritchie(hir_decl) => {
            Some(MajorRitchieHirDefn::new(db, path, hir_decl).into())
        }
        MajorFormHirDecl::Val(hir_decl) => Some(ValHirDefn::new(db, path, hir_decl).into()),
        MajorFormHirDecl::TypeAlias(_) => todo!(),
    }
}
