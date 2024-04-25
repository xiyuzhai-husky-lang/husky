use super::*;
use husky_entity_kind::TypeKind;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntityTreeJar)]
#[salsa::deref_id]
pub struct TypeSynNodePath(ItemSynNodePathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeSynNodePathData {
    disambiguated_item_path: DisambiguatedItemPath<TypePath>,
}

impl TypeSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: TypePath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Type(TypeSynNodePathData {
                disambiguated_item_path: registry.issue_maybe_ambiguous_path(path),
            })),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TypeSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Type(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db)
            .disambiguated_item_path
            .maybe_ambiguous_item_path
            .ident(db)
    }

    pub fn unambiguous_item_path(self, db: &::salsa::Db) -> Option<TypePath> {
        self.data(db).path()
    }

    pub fn ty_kind(self, db: &::salsa::Db) -> TypeKind {
        self.data(db)
            .disambiguated_item_path
            .maybe_ambiguous_item_path
            .ty_kind(db)
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a MajorItemSynNode {
        let module_path: ModulePath = self.module_path(db);
        // it's important to use presheet instead of sheet
        // otherwise cyclic when use all type variant paths
        let item_presheet = db.item_syn_tree_presheet(module_path);
        let Some(ItemSynNode::MajorItem(node)) = item_presheet.major_item_node(self.into()) else {
            unreachable!("should be some, must be some erros in library")
        };
        node
    }
}

impl TypeSynNodePathData {
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> TypeSynNodePath {
        TypeSynNodePath(id)
    }

    pub fn path(self) -> Option<TypePath> {
        self.disambiguated_item_path.unambiguous_item_path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.disambiguated_item_path
            .maybe_ambiguous_item_path
            .module_path(db)
    }

    pub fn ty_kind(self, db: &::salsa::Db) -> TypeKind {
        self.disambiguated_item_path
            .maybe_ambiguous_item_path
            .ty_kind(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        TypeSynNodePath(id).syn_node(db).ast_idx
    }
}

impl HasSynNodePath for TypePath {
    type SynNodePath = TypeSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TypeSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Type(TypeSynNodePathData {
                disambiguated_item_path: DisambiguatedItemPath::from_path(self),
            })),
        ))
    }
}

impl From<TypeSynNodePath> for ItemSynNodePath {
    fn from(id: TypeSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}
