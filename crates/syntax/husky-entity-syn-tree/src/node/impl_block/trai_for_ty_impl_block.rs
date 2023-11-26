use vec_like::SmallVecPairMap;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitForTypeImplBlockSynNodePath(ItemSynNodePathId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[salsa::as_id(jar = EntitySynTreeJar)]
pub struct TraitForTypeImplBlockSynNodePathData {
    path: TraitForTypeImplBlockPath,
}

impl TraitForTypeImplBlockSynNodePath {
    pub fn path(self) -> TraitForTypeImplBlockPath {
        self.path
    }

    pub fn trai_path(self, db: &::salsa::Db) -> TraitPath {
        self.path.trai_path(db)
    }

    pub fn ty_sketch(self, db: &::salsa::Db) -> TypeSketch {
        self.path.ty_sketch(db)
    }

    pub(crate) fn associated_items(
        self,
        db: &::salsa::Db,
    ) -> &[(
        Ident,
        TraitForTypeItemSynNodePath,
        TraitForTypeItemSynNodeData,
    )] {
        trai_for_ty_impl_block_items(db, self)
    }

    pub fn item_syn_node_paths<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = TraitForTypeItemSynNodePath> + 'a {
        self.associated_items(db)
            .iter()
            .copied()
            .map(|(_, syn_node_path, _)| syn_node_path)
    }

    pub(crate) fn syn_node(self, db: &::salsa::Db) -> TraitForTypeImplBlockSynNodeData {
        trai_for_ty_impl_block_syn_node(db, self)
    }
}

impl From<TraitForTypeImplBlockSynNodePath> for ItemSynNodePath {
    fn from(id: TraitForTypeImplBlockSynNodePath) -> Self {
        ItemSynNodePath::ImplBlock(id.into())
    }
}

impl HasSynNodePath for TraitForTypeImplBlockPath {
    type SynNodePath = TraitForTypeImplBlockSynNodePath;

    fn syn_node_path(self, _db: &::salsa::Db) -> Self::SynNodePath {
        TraitForTypeImplBlockSynNodePath(todo!())
        // { path: self })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct TraitForTypeImplBlockSynNodeData {
    syn_node_path: TraitForTypeImplBlockSynNodePath,
    ast_idx: AstIdx,
    impl_regional_token: ImplToken,
    trai_expr: MajorItemPathExprIdx,
    for_token: TokenIdx,
    ty_sketch_expr: SelfTypeSketchExpr,
    items: Option<ImplBlockItems>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SelfTypeSketchExpr {
    Path(MajorItemPathExprIdx),
    DeriveAny {
        pound_token: PoundToken,
        derive_token: DeriveToken,
        underscore_token: UnderscoreToken,
    },
}

impl TraitForTypeImplBlockSynNodeData {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        impl_regional_token: ImplToken,
        trai_expr: MajorItemPathExprIdx,
        trai_path: TraitPath,
        for_token: TokenIdx,
        ty_sketch_expr: SelfTypeSketchExpr,
        ty_sketch: TypeSketch,
        items: Option<ImplBlockItems>,
    ) -> Result<Self, ImplBlockIllForm> {
        // todo: check trai_path and ty_sketch
        // should check that if one of trai_path and ty_sketch must be always contained inside the crate
        Ok(TraitForTypeImplBlockSynNodeData {
            syn_node_path: TraitForTypeImplBlockSynNodePath(ItemSynNodePathId::new(
                db,
                ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePathData {
                        path: TraitForTypeImplBlockPath::new(
                            db,
                            registry,
                            module_path,
                            trai_path,
                            ty_sketch,
                        ),
                    },
                )),
            )),
            ast_idx,
            impl_regional_token,
            trai_expr,
            for_token,
            ty_sketch_expr,
            items,
        })
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.syn_node_path(db).path.module_path(db)
    }

    pub fn ty_sketch(self, db: &::salsa::Db) -> TypeSketch {
        self.syn_node_path(db).ty_sketch(db)
    }

    pub fn trai_path(self, db: &::salsa::Db) -> TraitPath {
        self.syn_node_path(db).path.trai_path(db)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn trai_for_ty_impl_block_syn_node(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> TraitForTypeImplBlockSynNodeData {
    let module_path = todo!(); //syn_node_path.module_path(db);
    let item_tree_sheet = db.item_syn_tree_sheet(module_path);
    item_tree_sheet.trai_for_ty_impl_block_syn_node(db, syn_node_path)
}

impl HasAssociatedItemPaths for TraitForTypeImplBlockPath {
    type AssociatedItemPath = TraitForTypeItemPath;

    fn associated_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssociatedItemPath)] {
        trai_for_ty_impl_block_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn trai_for_ty_impl_block_item_paths(
    db: &::salsa::Db,
    path: TraitForTypeImplBlockPath,
) -> SmallVecPairMap<Ident, TraitForTypeItemPath, 2> {
    path.syn_node_path(db)
        .associated_items(db)
        .iter()
        .filter_map(|(ident, syn_node_path, _)| Some((*ident, syn_node_path.path(db)?)))
        .collect()
}
