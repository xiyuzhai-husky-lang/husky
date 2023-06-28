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
pub enum ModuleItemNodeDefn {
    Type(TypeNodeDefn),
    Trait(TraitNodeDefn),
    Fugitive(FugitiveNodeDefn),
}

impl ModuleItemNodeDefn {
    pub fn node_decl(self, db: &dyn DefnDb) -> ModuleItemNodeDecl {
        match self {
            ModuleItemNodeDefn::Type(node_defn) => node_defn.node_decl(db).into(),
            ModuleItemNodeDefn::Trait(node_defn) => node_defn.node_decl(db).into(),
            ModuleItemNodeDefn::Fugitive(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            ModuleItemNodeDefn::Type(_) | ModuleItemNodeDefn::Trait(_) => None,
            ModuleItemNodeDefn::Fugitive(node_defn) => Some(node_defn.expr_region(db)),
        }
    }
}

impl HasNodeDefn for ModuleItemNodePath {
    type NodeDefn = ModuleItemNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        match self {
            ModuleItemNodePath::Trait(node_path) => node_path.node_defn(db).into(),
            ModuleItemNodePath::Type(node_path) => node_path.node_defn(db).into(),
            ModuleItemNodePath::Fugitive(node_path) => node_path.node_defn(db).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum ModuleItemDefn {
    Type(TypeDefn),
    Trait(TraitDefn),
    Fugitive(FugitiveDefn),
}

impl ModuleItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> ModuleItemDecl {
        match self {
            ModuleItemDefn::Type(defn) => defn.decl(db).into(),
            ModuleItemDefn::Trait(defn) => defn.decl(db).into(),
            ModuleItemDefn::Fugitive(defn) => defn.decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            ModuleItemDefn::Type(_) | ModuleItemDefn::Trait(_) => None,
            ModuleItemDefn::Fugitive(defn) => Some(defn.expr_region(db)),
        }
    }
}

impl HasDefn for ModuleItemPath {
    type Defn = ModuleItemDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        Ok(match self {
            ModuleItemPath::Type(path) => path.defn(db)?.into(),
            ModuleItemPath::Fugitive(path) => path.defn(db)?.into(),
            ModuleItemPath::Trait(path) => path.defn(db)?.into(),
        })
    }
}
