use super::*;
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::major_item::form::MajorFormPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct FormSynNodePath(ItemSynNodePathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormSynNodePathData {
    pub disambiguated_item_path: DisambiguatedItemPath<MajorFormPath>,
}

impl From<FormSynNodePath> for ItemSynNodePath {
    fn from(id: FormSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}

impl HasSynNodePath for MajorFormPath {
    type SynNodePath = FormSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        FormSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Form(FormSynNodePathData {
                disambiguated_item_path: DisambiguatedItemPath::from_path(self),
            })),
        ))
    }
}

impl FormSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: MajorFormPath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Form(FormSynNodePathData {
                disambiguated_item_path: registry.issue_maybe_ambiguous_path(path),
            })),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> FormSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Form(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db)
            .disambiguated_item_path
            .maybe_ambiguous_item_path
            .ident(db)
    }

    pub fn form_kind(self, db: &::salsa::Db) -> MajorFormKind {
        self.data(db)
            .disambiguated_item_path
            .maybe_ambiguous_item_path
            .major_form_kind(db)
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

impl salsa::DebugWithDb for FormSynNodePath {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        use std::fmt::Debug;

        f.write_str("FormSynNodePath(`")?;
        let disambiguated_item_path = self.data(db).disambiguated_item_path;
        disambiguated_item_path
            .maybe_ambiguous_item_path
            .show_aux(f, db)?;
        f.write_str("`, `")?;
        self.data(db)
            .disambiguated_item_path
            .maybe_ambiguous_item_path
            .major_form_kind(db)
            .fmt(f)?;
        f.write_fmt(format_args!(
            "`, ({}))",
            disambiguated_item_path.disambiguator
        ))
    }
}

impl FormSynNodePathData {
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> FormSynNodePath {
        FormSynNodePath(id)
    }

    pub fn path(self) -> Option<MajorFormPath> {
        self.disambiguated_item_path.unambiguous_item_path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.disambiguated_item_path
            .maybe_ambiguous_item_path
            .module_path(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        FormSynNodePath(id).syn_node(db).ast_idx
    }
}
