mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;
use husky_entity_path::MajarItemPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
#[enum_class::from_variants]
pub enum MajorItemSynNodePath {
    Trait(TraitSynNodePath),
    Type(TypeSynNodePath),
    Fugitive(FugitiveSynNodePath),
}

impl MajorItemSynNodePath {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemNodeRegistry,
        path: MajarItemPath,
    ) -> Self {
        match path {
            MajarItemPath::Type(path) => TypeSynNodePath::new(db, registry, path).into(),
            MajarItemPath::Trait(path) => TraitSynNodePath::new(db, registry, path).into(),
            MajarItemPath::Fugitive(path) => FugitiveSynNodePath::new(db, registry, path).into(),
        }
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<MajarItemPath> {
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

    pub fn ident(self, db: &dyn EntitySynTreeDb) -> Ident {
        todo!("")
        // self.path(db).ident(db)
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        match self {
            MajorItemSynNodePath::Trait(node) => node.module_path(db),
            MajorItemSynNodePath::Type(node) => node.module_path(db),
            MajorItemSynNodePath::Fugitive(node) => node.module_path(db),
        }
    }

    pub fn node(self, db: &dyn EntitySynTreeDb) -> MajorItemSynNode {
        todo!()
    }
}

impl HasSynNodePath for MajarItemPath {
    type SynNodePath = MajorItemSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        match self {
            MajarItemPath::Type(path) => path.syn_node_path(db).into(),
            MajarItemPath::Trait(path) => path.syn_node_path(db).into(),
            MajarItemPath::Fugitive(path) => path.syn_node_path(db).into(),
        }
    }
}

// todo: change this to enum and create FugitiveNode etc.
#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct MajorItemSynNode {
    #[id]
    pub syn_node_path: MajorItemSynNodePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
    pub block: DefnBlock,
}

impl MajorItemSynNode {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemNodeRegistry,
        module_item_path: MajarItemPath,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        block: DefnBlock,
    ) -> Self {
        MajorItemSynNode::new_inner(
            db,
            MajorItemSynNodePath::new(db, registry, module_item_path),
            visibility,
            ast_idx,
            ident_token,
            block,
        )
    }

    /// only gives a path when valid
    pub fn unambiguous_path(self, db: &dyn EntitySynTreeDb) -> Option<MajarItemPath> {
        self.syn_node_path(db).path(db)
    }
}
