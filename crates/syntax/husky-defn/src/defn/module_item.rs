mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum ModuleItemDefn {
    Type(TypeDefn),
    Trait(TraitDefn),
    Fugitive(FugitiveDefn),
}

impl ModuleItemDefn {
    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            ModuleItemDefn::Type(_) | ModuleItemDefn::Trait(_) => None,
            ModuleItemDefn::Fugitive(defn) => Some(defn.expr_region(db)),
        }
    }
}

impl HasDefn for ModuleItemDecl {
    type Defn = ModuleItemDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            ModuleItemDecl::Type(decl) => decl.defn(db).into(),
            ModuleItemDecl::Fugitive(decl) => decl.defn(db).into(),
            ModuleItemDecl::Trait(decl) => decl.defn(db).into(),
        }
    }
}
