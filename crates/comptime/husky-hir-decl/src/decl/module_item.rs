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
pub enum ModuleItemHirDecl {
    Type(TypeHirDecl),
    Trait(TraitHirDecl),
    Fugitive(FugitiveHirDecl),
}

impl ModuleItemHirDecl {
    pub fn template_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [EtherealTemplateParameter] {
        match self {
            ModuleItemHirDecl::Type(decl) => decl.template_parameters(db),
            ModuleItemHirDecl::Fugitive(decl) => decl.template_parameters(db),
            ModuleItemHirDecl::Trait(decl) => decl.template_parameters(db),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            ModuleItemHirDecl::Type(decl) => decl.hir_expr_region(db).into(),
            ModuleItemHirDecl::Fugitive(decl) => decl.hir_expr_region(db).into(),
            ModuleItemHirDecl::Trait(decl) => decl.hir_expr_region(db).into(),
        }
    }

    pub fn path(self, db: &dyn HirDeclDb) -> ModuleItemPath {
        match self {
            ModuleItemHirDecl::Type(decl) => decl.path(db).into(),
            ModuleItemHirDecl::Fugitive(decl) => decl.path(db).into(),
            ModuleItemHirDecl::Trait(decl) => decl.path(db).into(),
        }
    }
}

impl HasHirDecl for ModuleItemPath {
    type HirDecl = ModuleItemHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        Some(match self {
            ModuleItemPath::Type(path) => path.hir_decl(db)?.into(),
            ModuleItemPath::Trait(path) => path.hir_decl(db)?.into(),
            ModuleItemPath::Fugitive(path) => path.hir_decl(db)?.into(),
        })
    }
}
