mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum ModuleItemHirDefn {
    Type(TypeHirDefn),
    Trait(TraitHirDefn),
    Fugitive(FugitiveHirDefn),
}

impl ModuleItemHirDefn {
    pub fn hir_decl(self, db: &dyn HirDefnDb) -> ModuleItemHirDecl {
        match self {
            ModuleItemHirDefn::Type(hir_defn) => hir_defn.hir_decl(db).into(),
            ModuleItemHirDefn::Trait(hir_defn) => hir_defn.hir_decl(db).into(),
            ModuleItemHirDefn::Fugitive(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            ModuleItemHirDefn::Type(_) | ModuleItemHirDefn::Trait(_) => None,
            ModuleItemHirDefn::Fugitive(hir_defn) => Some(hir_defn.hir_expr_region(db)),
        }
    }
}

impl HasHirDefn for ModuleItemPath {
    type HirDefn = ModuleItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        match self {
            ModuleItemPath::Type(path) => path.hir_defn(db).into(),
            ModuleItemPath::Fugitive(path) => path.hir_defn(db).into(),
            ModuleItemPath::Trait(path) => path.hir_defn(db).into(),
        }
    }
}
