use husky_entity_kind::FugitiveKind;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct FugitiveSynNodePath(ItemSynNodePathId);

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FugitiveSynNodePathData {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<FugitivePath>,
}

impl From<FugitiveSynNodePath> for ItemSynNodePath {
    fn from(id: FugitiveSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}

impl HasSynNodePath for FugitivePath {
    type SynNodePath = FugitiveSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        FugitiveSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Fugitive(
                FugitiveSynNodePathData {
                    maybe_ambiguous_path: MaybeAmbiguousPath::from_path(self),
                },
            )),
        ))
    }
}

impl FugitiveSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: FugitivePath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Fugitive(
                FugitiveSynNodePathData {
                    maybe_ambiguous_path: registry.issue_maybe_ambiguous_path(path),
                },
            )),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> FugitiveSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Fugitive(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).maybe_ambiguous_path.path.ident(db)
    }

    pub fn fugitive_kind(self, db: &::salsa::Db) -> FugitiveKind {
        self.data(db).maybe_ambiguous_path.path.fugitive_kind(db)
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a MajorItemSynNode {
        let module_path = self.module_path(db);
        let item_sheet = module_path.item_tree_sheet(db);
        match item_sheet
            .major_item_node(self.into())
            .expect("should be some")
        {
            ItemSynNode::MajorItem(node) => node,
            _ => unreachable!(),
        }
    }
}

impl FugitiveSynNodePathData {
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> FugitiveSynNodePath {
        FugitiveSynNodePath(id)
    }

    pub fn path(self) -> Option<FugitivePath> {
        self.maybe_ambiguous_path.unambiguous_path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.maybe_ambiguous_path.path.module_path(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        FugitiveSynNodePath(id).syn_node(db).ast_idx
    }
}
