use original_error::IntoError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct IllFormedImplBlockNodePath {
    path: IllFormedImplBlockPath,
}

impl salsa::AsId for IllFormedImplBlockNodePath {
    fn as_id(self) -> salsa::Id {
        self.path.as_id()
    }

    fn from_id(id: salsa::Id) -> Self {
        IllFormedImplBlockNodePath {
            path: IllFormedImplBlockPath::from_id(id),
        }
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for IllFormedImplBlockNodePath
where
    DB: ?Sized + salsa::DbWithJar<EntityPathJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

impl IllFormedImplBlockNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path.module_path(db)
    }

    pub fn item_node_paths(self, db: &dyn EntityTreeDb) -> &[IllFormedItemNodePath] {
        // ad hoc
        &[]
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> IllFormedImplBlockNode {
        ill_formed_impl_block_node(db, self)
    }
}

impl From<IllFormedImplBlockNodePath> for EntityNodePath {
    fn from(id: IllFormedImplBlockNodePath) -> Self {
        EntityNodePath::ImplBlock(id.into())
    }
}

impl HasNodePath for IllFormedImplBlockPath {
    type NodePath = IllFormedImplBlockNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        IllFormedImplBlockNodePath { path: self }
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct IllFormedImplBlockNode {
    #[id]
    pub node_path: IllFormedImplBlockNodePath,
    pub impl_token: ImplToken,
    pub ast_idx: AstIdx,
    pub items: Option<ImplBlockItems>,
    #[return_ref]
    pub ill_form: ImplBlockIllForm,
}

impl IllFormedImplBlockNode {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        impl_token: ImplToken,
        module: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>,
        ill_form: ImplBlockIllForm,
    ) -> Self {
        IllFormedImplBlockNode::new_inner(
            db,
            IllFormedImplBlockNodePath {
                path: IllFormedImplBlockPath::new(db, registry, module),
            },
            impl_token,
            ast_idx,
            items,
            ill_form,
        )
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ImplBlockIllForm {
    #[error("unmatched angle bras")]
    UnmatchedAngleBras,
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("principal path expr error")]
    MajorPath(#[from] MajorPathExprError),
    #[error("MissingFor")]
    MissingForKeyword,
    #[error("ExpectTypePathAfterFor")]
    ExpectTypePathAfterForKeyword,
    #[error("expected `derive` identifier")]
    ExpectedDeriveIdent(TokenStreamState),
}

impl IntoError for ImplBlockIllForm {
    type Error = Self;
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn ill_formed_impl_block_node(
    db: &dyn EntityTreeDb,
    node_path: IllFormedImplBlockNodePath,
) -> IllFormedImplBlockNode {
    let module_path = node_path.module_path(db);
    let entity_tree_sheet = db.entity_tree_sheet(module_path).expect("valid module");
    entity_tree_sheet.ill_formed_impl_block_node(db, node_path)
}
