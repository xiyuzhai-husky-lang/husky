mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum ModuleItemSynNodeDefn {
    Type(TypeSynNodeDefn),
    Trait(TraitSynNodeDefn),
    Fugitive(FugitiveSynNodeDefn),
}

impl ModuleItemSynNodeDefn {
    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> ModuleItemSynNodeDecl {
        match self {
            ModuleItemSynNodeDefn::Type(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            ModuleItemSynNodeDefn::Trait(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            ModuleItemSynNodeDefn::Fugitive(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
        }
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            ModuleItemSynNodeDefn::Type(_) | ModuleItemSynNodeDefn::Trait(_) => None,
            ModuleItemSynNodeDefn::Fugitive(syn_node_defn) => Some(syn_node_defn.expr_region(db)),
        }
    }
}

impl HasSynNodeDefn for ModuleItemSynNodePath {
    type NodeDefn = ModuleItemSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        match self {
            ModuleItemSynNodePath::Trait(syn_node_path) => syn_node_path.syn_node_defn(db).into(),
            ModuleItemSynNodePath::Type(syn_node_path) => syn_node_path.syn_node_defn(db).into(),
            ModuleItemSynNodePath::Fugitive(syn_node_path) => {
                syn_node_path.syn_node_defn(db).into()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum ModuleItemDefn {
    Type(TypeDefn),
    Trait(TraitSynDefn),
    Fugitive(FugitiveSynDefn),
}

impl ModuleItemDefn {
    pub fn decl(self, db: &dyn SynDefnDb) -> ModuleItemSynDecl {
        match self {
            ModuleItemDefn::Type(defn) => defn.decl(db).into(),
            ModuleItemDefn::Trait(defn) => defn.decl(db).into(),
            ModuleItemDefn::Fugitive(defn) => defn.decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            ModuleItemDefn::Type(_) | ModuleItemDefn::Trait(_) => None,
            ModuleItemDefn::Fugitive(defn) => Some(defn.expr_region(db)),
        }
    }
}

impl HasSynDefn for ModuleItemPath {
    type SynDefn = ModuleItemDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        Ok(match self {
            ModuleItemPath::Type(path) => path.syn_defn(db)?.into(),
            ModuleItemPath::Fugitive(path) => path.syn_defn(db)?.into(),
            ModuleItemPath::Trait(path) => path.syn_defn(db)?.into(),
        })
    }
}
