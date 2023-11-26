mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb, jar = HirDeclJar)]
#[enum_class::from_variants]
pub enum MajorItemHirDecl {
    Type(TypeHirDecl),
    Trait(TraitHirDecl),
    Fugitive(FugitiveHirDecl),
}

impl MajorItemHirDecl {
    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [HirTemplateParameter] {
        match self {
            MajorItemHirDecl::Type(decl) => decl.template_parameters(db),
            MajorItemHirDecl::Fugitive(decl) => decl.template_parameters(db),
            MajorItemHirDecl::Trait(decl) => decl.template_parameters(db),
        }
    }

    // pub fn hir_expr_region(self, db: &::salsa::Db,) -> Option<HirExprRegion> {
    //     match self {
    //         MajorItemHirDecl::Type(decl) => Some(decl.hir_expr_region(db)),
    //         MajorItemHirDecl::Fugitive(decl) => Some(decl.hir_expr_region(db)),
    //         MajorItemHirDecl::Trait(decl) => None,
    //     }
    // }

    pub fn path(self, db: &::salsa::Db) -> MajorItemPath {
        match self {
            MajorItemHirDecl::Type(decl) => decl.path(db).into(),
            MajorItemHirDecl::Fugitive(decl) => decl.path(db).into(),
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
            MajorItemPath::Fugitive(path) => path.hir_decl(db)?.into(),
        })
    }
}
