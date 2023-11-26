mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;
use husky_entity_path::MajorItemPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[enum_class::from_variants]
pub enum MajorItemSynNodePath {
    Trait(TraitSynNodePath),
    Type(TypeSynNodePath),
    Fugitive(FugitiveSynNodePath),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[enum_class::from_variants]
pub enum MajorItemSynNodePathData {
    Trait(TraitSynNodePathData),
    Type(TypeSynNodePathData),
    Fugitive(FugitiveSynNodePathData),
}

impl MajorItemSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: MajorItemPath,
    ) -> Self {
        match path {
            MajorItemPath::Type(path) => TypeSynNodePath::new(db, registry, path).into(),
            MajorItemPath::Trait(path) => TraitSynNodePath::new(db, registry, path).into(),
            MajorItemPath::Fugitive(path) => FugitiveSynNodePath::new(db, registry, path).into(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> Option<MajorItemPath> {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => syn_node_path
                .maybe_ambiguous_path(db)
                .unambiguous_path()
                .map(Into::into),
            MajorItemSynNodePath::Type(syn_node_path) => syn_node_path
                .maybe_ambiguous_path(db)
                .unambiguous_path()
                .map(Into::into),
            MajorItemSynNodePath::Fugitive(syn_node_path) => syn_node_path
                .maybe_ambiguous_path(db)
                .unambiguous_path()
                .map(Into::into),
        }
    }

    pub fn ident(self, _db: &::salsa::Db) -> Ident {
        todo!("")
        // self.path(db).ident(db)
    }

    pub(crate) fn syn_node(self, _db: &::salsa::Db) -> MajorItemSynNodeData {
        todo!()
    }

    pub(crate) fn attrs(self, db: &::salsa::Db) -> &[(AttrSynNodePath, AttrSynNodeData)] {
        // ad hoc
        match self {
            MajorItemSynNodePath::Trait(_) => &[],
            MajorItemSynNodePath::Type(syn_node_path) => syn_node_path.attr_syn_nodes(db),
            MajorItemSynNodePath::Fugitive(_) => &[],
        }
    }
}

impl HasSynNodePath for MajorItemPath {
    type SynNodePath = MajorItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        match self {
            MajorItemPath::Type(path) => path.syn_node_path(db).into(),
            MajorItemPath::Trait(path) => path.syn_node_path(db).into(),
            MajorItemPath::Fugitive(path) => path.syn_node_path(db).into(),
        }
    }
}

// todo: change this to enum and create FugitiveNode etc.
#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub(crate) struct MajorItemSynNodeData {
    #[id]
    pub syn_node_path: MajorItemSynNodePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
    pub block: DefnBlock,
}

impl MajorItemSynNodeData {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        module_item_path: MajorItemPath,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        block: DefnBlock,
    ) -> Self {
        MajorItemSynNodeData::new_inner(
            db,
            MajorItemSynNodePath::new(db, registry, module_item_path),
            visibility,
            ast_idx,
            ident_token,
            block,
        )
    }

    /// only gives a path when valid
    pub fn unambiguous_path(self, db: &::salsa::Db) -> Option<MajorItemPath> {
        self.syn_node_path(db).path(db)
    }
}
