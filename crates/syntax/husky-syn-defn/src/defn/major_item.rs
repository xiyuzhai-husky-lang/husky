mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum MajorItemSynNodeDefn {
    Type(TypeSynNodeDefn),
    Trait(TraitSynNodeDefn),
    Fugitive(FugitiveSynNodeDefn),
}

impl MajorItemSynNodeDefn {
    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> MajorItemSynNodeDecl {
        match self {
            MajorItemSynNodeDefn::Type(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            MajorItemSynNodeDefn::Trait(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            MajorItemSynNodeDefn::Fugitive(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &dyn SynDefnDb,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            MajorItemSynNodeDefn::Type(_) | MajorItemSynNodeDefn::Trait(_) => None,
            MajorItemSynNodeDefn::Fugitive(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
        }
    }
}

impl HasSynNodeDefn for MajorItemSynNodePath {
    type SynNodeDefn = MajorItemSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => syn_node_path.syn_node_defn(db).into(),
            MajorItemSynNodePath::Type(syn_node_path) => syn_node_path.syn_node_defn(db).into(),
            MajorItemSynNodePath::Fugitive(syn_node_path) => syn_node_path.syn_node_defn(db).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum MajorItemSynDefn {
    Type(TypeSynDefn),
    Trait(TraitSynDefn),
    Fugitive(FugitiveSynDefn),
}

impl MajorItemSynDefn {
    pub fn syn_decl(self, db: &dyn SynDefnDb) -> MajorItemSynDecl {
        match self {
            MajorItemSynDefn::Type(defn) => defn.decl(db).into(),
            MajorItemSynDefn::Trait(defn) => defn.decl(db).into(),
            MajorItemSynDefn::Fugitive(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn SynDefnDb) -> MajorItemPath {
        match self {
            MajorItemSynDefn::Type(syn_defn) => syn_defn.path(db).into(),
            MajorItemSynDefn::Trait(syn_defn) => syn_defn.path(db).into(),
            MajorItemSynDefn::Fugitive(syn_defn) => syn_defn.path(db).into(),
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &dyn SynDefnDb,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            MajorItemSynDefn::Type(_) | MajorItemSynDefn::Trait(_) => None,
            MajorItemSynDefn::Fugitive(defn) => defn.body_with_syn_expr_region(db),
        }
    }
}

impl HasSynDefn for MajorItemPath {
    type SynDefn = MajorItemSynDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        Ok(match self {
            MajorItemPath::Type(path) => path.syn_defn(db)?.into(),
            MajorItemPath::Fugitive(path) => path.syn_defn(db)?.into(),
            MajorItemPath::Trait(path) => path.syn_defn(db)?.into(),
        })
    }
}
