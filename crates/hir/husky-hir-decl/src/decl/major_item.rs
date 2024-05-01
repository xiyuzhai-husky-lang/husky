pub mod form;
mod trai;
mod ty;

pub use self::form::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorItemHirDecl {
    Type(TypeHirDecl),
    Trait(TraitHirDecl),
    Form(MajorFormHirDecl),
}

impl MajorItemHirDecl {
    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> Option<&'a HirTemplateParameters> {
        match self {
            MajorItemHirDecl::Type(decl) => Some(decl.template_parameters(db)),
            MajorItemHirDecl::Form(decl) => decl.template_parameters(db),
            MajorItemHirDecl::Trait(decl) => Some(decl.template_parameters(db)),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            MajorItemHirDecl::Type(decl) => decl.hir_expr_region(db),
            MajorItemHirDecl::Form(decl) => decl.hir_expr_region(db),
            MajorItemHirDecl::Trait(decl) => decl.hir_eager_expr_region(db).into(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> MajorItemPath {
        match self {
            MajorItemHirDecl::Type(decl) => decl.path(db).into(),
            MajorItemHirDecl::Form(decl) => decl.path(db).into(),
            MajorItemHirDecl::Trait(decl) => decl.path(db).into(),
        }
    }
}

impl HasHirDecl for MajorItemPath {
    type HirDecl = MajorItemHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        Some(match self {
            MajorItemPath::Type(path) => path.hir_decl(db)?.into(),
            MajorItemPath::Trait(path) => path.hir_decl(db)?.into(),
            MajorItemPath::Form(path) => path.hir_decl(db)?.into(),
        })
    }
}
