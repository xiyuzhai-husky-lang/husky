use husky_entity_kind::FugitiveKind;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
pub struct FugitiveSynNodePath(ItemSynNodePathId);

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

    pub fn fugitive_kind(self, db: &::salsa::Db) -> FugitiveKind {
        self.data(db).maybe_ambiguous_path.path.fugitive_kind(db)
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a MajorItemSynNode {
        let module_path: ModulePath = todo!(); //syn_node_path.module_path(db);
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
