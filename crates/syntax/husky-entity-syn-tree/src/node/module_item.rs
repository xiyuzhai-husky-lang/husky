mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;
use husky_entity_path::ModuleItemPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
#[enum_class::from_variants]
pub enum ModuleItemSynNodePath {
    Trait(TraitSynNodePath),
    Type(TypeSynNodePath),
    Fugitive(FugitiveSynNodePath),
}

impl ModuleItemSynNodePath {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut EntityNodeRegistry,
        path: ModuleItemPath,
    ) -> Self {
        match path {
            ModuleItemPath::Type(path) => TypeSynNodePath::new(db, registry, path).into(),
            ModuleItemPath::Trait(path) => TraitSynNodePath::new(db, registry, path).into(),
            ModuleItemPath::Fugitive(path) => FugitiveSynNodePath::new(db, registry, path).into(),
        }
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<ModuleItemPath> {
        match self {
            ModuleItemSynNodePath::Trait(syn_node_path) => syn_node_path
                .maybe_ambiguous_path(db)
                .unambiguous_path()
                .map(Into::into),
            ModuleItemSynNodePath::Type(syn_node_path) => syn_node_path
                .maybe_ambiguous_path(db)
                .unambiguous_path()
                .map(Into::into),
            ModuleItemSynNodePath::Fugitive(syn_node_path) => syn_node_path
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
            ModuleItemSynNodePath::Trait(node) => node.module_path(db),
            ModuleItemSynNodePath::Type(node) => node.module_path(db),
            ModuleItemSynNodePath::Fugitive(node) => node.module_path(db),
        }
    }

    pub fn node(self, db: &dyn EntitySynTreeDb) -> ModuleItemSynNode {
        todo!()
    }
}

impl HasSynNodePath for ModuleItemPath {
    type SynNodePath = ModuleItemSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        match self {
            ModuleItemPath::Type(path) => path.syn_node_path(db).into(),
            ModuleItemPath::Trait(path) => path.syn_node_path(db).into(),
            ModuleItemPath::Fugitive(path) => path.syn_node_path(db).into(),
        }
    }
}

// todo: change this to enum and create FugitiveNode etc.
#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct ModuleItemSynNode {
    #[id]
    pub syn_node_path: ModuleItemSynNodePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
    pub block: DefnBlock,
}

impl ModuleItemSynNode {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut EntityNodeRegistry,
        module_item_path: ModuleItemPath,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        block: DefnBlock,
    ) -> Self {
        ModuleItemSynNode::new_inner(
            db,
            ModuleItemSynNodePath::new(db, registry, module_item_path),
            visibility,
            ast_idx,
            ident_token,
            block,
        )
    }

    /// only gives a path when valid
    pub fn unambiguous_path(self, db: &dyn EntitySynTreeDb) -> Option<ModuleItemPath> {
        self.syn_node_path(db).path(db)
    }
}
