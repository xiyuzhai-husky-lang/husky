mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum ModuleItemHirNodeDefn {
    Type(TypeHirNodeDefn),
    Trait(TraitHirNodeDefn),
    Fugitive(FugitiveHirNodeDefn),
}

impl ModuleItemHirNodeDefn {
    pub fn syn_node_decl(self, db: &dyn HirDefnDb) -> ModuleItemHirNodeDecl {
        match self {
            ModuleItemHirNodeDefn::Type(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            ModuleItemHirNodeDefn::Trait(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            ModuleItemHirNodeDefn::Fugitive(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
        }
    }

    pub fn expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            ModuleItemHirNodeDefn::Type(_) | ModuleItemHirNodeDefn::Trait(_) => None,
            ModuleItemHirNodeDefn::Fugitive(syn_node_defn) => Some(syn_node_defn.expr_region(db)),
        }
    }
}

impl HasHirNodeDefn for ModuleItemHirNodePath {
    type NodeDefn = ModuleItemHirNodeDefn;

    fn syn_node_defn(self, db: &dyn HirDefnDb) -> Self::NodeDefn {
        match self {
            ModuleItemHirNodePath::Trait(syn_node_path) => syn_node_path.syn_node_defn(db).into(),
            ModuleItemHirNodePath::Type(syn_node_path) => syn_node_path.syn_node_defn(db).into(),
            ModuleItemHirNodePath::Fugitive(syn_node_path) => {
                syn_node_path.syn_node_defn(db).into()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum ModuleItemHirDefn {
    Type(TypeDefn),
    Trait(TraitHirDefn),
    Fugitive(FugitiveDefn),
}

impl ModuleItemHirDefn {
    pub fn decl(self, db: &dyn HirDefnDb) -> ModuleItemDecl {
        match self {
            ModuleItemHirDefn::Type(defn) => defn.decl(db).into(),
            ModuleItemHirDefn::Trait(defn) => defn.decl(db).into(),
            ModuleItemHirDefn::Fugitive(defn) => defn.decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            ModuleItemHirDefn::Type(_) | ModuleItemHirDefn::Trait(_) => None,
            ModuleItemHirDefn::Fugitive(defn) => Some(defn.expr_region(db)),
        }
    }
}

impl HasHirDefn for ModuleItemPath {
    type HirDefn = ModuleItemHirDefn;

    fn syn_defn(self, db: &dyn HirDefnDb) -> HirDefnResult<Self::HirDefn> {
        Ok(match self {
            ModuleItemPath::Type(path) => path.syn_defn(db)?.into(),
            ModuleItemPath::Fugitive(path) => path.syn_defn(db)?.into(),
            ModuleItemPath::Trait(path) => path.syn_defn(db)?.into(),
        })
    }
}
