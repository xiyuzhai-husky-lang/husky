mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum MajorItemHirDecl {
    Type(TypeHirDecl),
    Trait(TraitHirDecl),
    Fugitive(FugitiveHirDecl),
}

impl MajorItemHirDecl {
    pub fn template_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [HirTemplateParameter] {
        match self {
            MajorItemHirDecl::Type(decl) => decl.template_parameters(db),
            MajorItemHirDecl::Fugitive(decl) => decl.template_parameters(db),
            MajorItemHirDecl::Trait(decl) => decl.template_parameters(db),
        }
    }

    // pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> Option<HirExprRegion> {
    //     match self {
    //         MajorItemHirDecl::Type(decl) => Some(decl.hir_expr_region(db)),
    //         MajorItemHirDecl::Fugitive(decl) => Some(decl.hir_expr_region(db)),
    //         MajorItemHirDecl::Trait(decl) => None,
    //     }
    // }

    pub fn path(self, db: &dyn HirDeclDb) -> MajarItemPath {
        match self {
            MajorItemHirDecl::Type(decl) => decl.path(db).into(),
            MajorItemHirDecl::Fugitive(decl) => decl.path(db).into(),
            MajorItemHirDecl::Trait(decl) => decl.path(db).into(),
        }
    }
}

impl HasHirDecl for MajarItemPath {
    type HirDecl = MajorItemHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        Some(match self {
            MajarItemPath::Type(path) => path.hir_decl(db)?.into(),
            MajarItemPath::Trait(path) => path.hir_decl(db)?.into(),
            MajarItemPath::Fugitive(path) => path.hir_decl(db)?.into(),
        })
    }
}
