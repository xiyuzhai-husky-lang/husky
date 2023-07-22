use vec_like::SmallVecPairMap;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct TraitForTypeImplBlockSynNodePath {
    path: TraitForTypeImplBlockPath,
}

impl salsa::AsId for TraitForTypeImplBlockSynNodePath {
    fn as_id(self) -> salsa::Id {
        self.path.as_id()
    }

    fn from_id(id: salsa::Id) -> Self {
        TraitForTypeImplBlockSynNodePath {
            path: TraitForTypeImplBlockPath::from_id(id),
        }
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for TraitForTypeImplBlockSynNodePath
where
    DB: ?Sized + salsa::DbWithJar<EntityPathJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

impl TraitForTypeImplBlockSynNodePath {
    pub fn path(self) -> TraitForTypeImplBlockPath {
        self.path
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.path.module_path(db)
    }

    pub fn trai_path(self, db: &dyn EntitySynTreeDb) -> TraitPath {
        self.path.trai_path(db)
    }

    pub fn ty_sketch(self, db: &dyn EntitySynTreeDb) -> TypeSketch {
        self.path.ty_sketch(db)
    }

    pub fn items(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> &[(Ident, TraitForTypeItemSynNodePath, TraitForTypeItemSynNode)] {
        trai_for_ty_impl_block_items(db, self)
    }

    pub fn item_node_paths<'a>(
        self,
        db: &'a dyn EntitySynTreeDb,
    ) -> impl Iterator<Item = TraitForTypeItemSynNodePath> + 'a {
        self.items(db)
            .iter()
            .copied()
            .map(|(_, syn_node_path, _)| syn_node_path)
    }

    pub fn node(self, db: &dyn EntitySynTreeDb) -> TraitForTypeImplBlockSynNode {
        trai_for_ty_impl_block_syn_node(db, self)
    }
}

impl From<TraitForTypeImplBlockSynNodePath> for EntitySynNodePath {
    fn from(id: TraitForTypeImplBlockSynNodePath) -> Self {
        EntitySynNodePath::ImplBlock(id.into())
    }
}

impl HasSynNodePath for TraitForTypeImplBlockPath {
    type SynNodePath = TraitForTypeImplBlockSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        TraitForTypeImplBlockSynNodePath { path: self }
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TraitForTypeImplBlockSynNode {
    #[id]
    pub syn_node_path: TraitForTypeImplBlockSynNodePath,
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    pub trai_expr: ModuleItemPathExprIdx,
    pub for_token: TokenIdx,
    pub ty_sketch_expr: SelfTypeSketchExpr,
    pub items: Option<ImplBlockItems>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SelfTypeSketchExpr {
    Path(ModuleItemPathExprIdx),
    DeriveAny {
        at_token: AtToken,
        derive_token: DeriveToken,
        underscore_token: UnderscoreToken,
    },
}

impl TraitForTypeImplBlockSynNode {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        impl_token: ImplToken,
        trai_expr: ModuleItemPathExprIdx,
        trai_path: TraitPath,
        for_token: TokenIdx,
        ty_sketch_expr: SelfTypeSketchExpr,
        ty_sketch: TypeSketch,
        items: Option<ImplBlockItems>,
    ) -> Result<Self, ImplBlockIllForm> {
        // todo: check trai_path and ty_sketch
        // should check that if one of trai_path and ty_sketch must be always contained inside the crate
        Ok(TraitForTypeImplBlockSynNode::new_inner(
            db,
            TraitForTypeImplBlockSynNodePath {
                path: TraitForTypeImplBlockPath::new(
                    db,
                    registry,
                    module_path,
                    trai_path,
                    ty_sketch,
                ),
            },
            ast_idx,
            impl_token,
            trai_expr,
            for_token,
            ty_sketch_expr,
            items,
        ))
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.syn_node_path(db).path.module_path(db)
    }

    pub fn ty_sketch(self, db: &dyn EntitySynTreeDb) -> TypeSketch {
        self.syn_node_path(db).path.ty_sketch(db)
    }

    pub fn trai_path(self, db: &dyn EntitySynTreeDb) -> TraitPath {
        self.syn_node_path(db).path.trai_path(db)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn trai_for_ty_impl_block_syn_node(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> TraitForTypeImplBlockSynNode {
    let module_path = syn_node_path.module_path(db);
    let entity_tree_sheet = db.entity_syn_tree_sheet(module_path).expect("valid module");
    entity_tree_sheet.trai_for_ty_impl_block_syn_node(db, syn_node_path)
}

impl HasItemPaths for TraitForTypeImplBlockPath {
    type ItemPath = TraitForTypeItemPath;

    fn item_paths(self, db: &dyn EntitySynTreeDb) -> &[(Ident, Self::ItemPath)] {
        trai_for_ty_impl_block_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn trai_for_ty_impl_block_item_paths(
    db: &dyn EntitySynTreeDb,
    path: TraitForTypeImplBlockPath,
) -> SmallVecPairMap<Ident, TraitForTypeItemPath, 2> {
    path.syn_node_path(db)
        .items(db)
        .iter()
        .filter_map(|(ident, syn_node_path, _)| Some((*ident, syn_node_path.path(db)?)))
        .collect()
}
