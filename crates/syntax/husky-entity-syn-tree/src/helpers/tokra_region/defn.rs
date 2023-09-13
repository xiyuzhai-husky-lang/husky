use idx_arena::ArenaRef;

use super::*;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct DefnTokraRegion {
    #[return_ref]
    tokens: Vec<TokenData>,
    #[return_ref]
    ast_arena: DefnAstArena,
}

impl DefnTokraRegion {
    pub fn data<'a>(self, db: &'a dyn EntitySynTreeDb) -> DefnTokraRegionData<'a> {
        DefnTokraRegionData {
            tokens: self.tokens(db),
            ast_arena: self.ast_arena(db).to_ref(),
        }
    }
}

pub struct DefnTokraRegionData<'a> {
    tokens: &'a [TokenData],
    ast_arena: DefnAstArenaRef<'a>,
}

impl<'a> std::ops::Index<RegionalTokenIdx> for DefnTokraRegionData<'a> {
    type Output = TokenData;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[idx.index()]
    }
}

pub(super) fn defn_token_region(
    syn_node_path: ItemSynNodePath,
    db: &dyn EntitySynTreeDb,
) -> Option<DefnTokraRegion> {
    match syn_node_path {
        ItemSynNodePath::Submodule(_) => None,
        ItemSynNodePath::MajorItem(_) => todo!(),
        ItemSynNodePath::TypeVariant(_) => todo!(),
        ItemSynNodePath::ImplBlock(_) => todo!(),
        ItemSynNodePath::AssociatedItem(_) => todo!(),
        ItemSynNodePath::Decr(_) => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DefnAst {}

pub type DefnAstArena = Arena<DefnAst>;
pub type DefnAstArenaRef<'a> = ArenaRef<'a, DefnAst>;
pub type DefnAstArenaIdx = ArenaIdx<DefnAst>;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct SynDefnTokraRegionSourceMap {
    token_region_base: TokenRegionBase,
}

pub trait HasSynDefnTokraRegion: for<'a> HasModulePath<dyn EntitySynTreeDb + 'a> + Copy {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion>;
    // use this only when necessary
    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap>;
}

impl HasSynDefnTokraRegion for ItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => syn_node_path.syn_defn_tokra_region(db),
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.syn_defn_tokra_region(db),
            ItemSynNodePath::TypeVariant(syn_node_path) => syn_node_path.syn_defn_tokra_region(db),
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.syn_defn_tokra_region(db),
            ItemSynNodePath::AssociatedItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region(db)
            }
            ItemSynNodePath::Decr(syn_node_path) => syn_node_path.syn_defn_tokra_region(db),
        }
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::MajorItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::TypeVariant(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::ImplBlock(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::AssociatedItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::Decr(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
        }
    }
}

impl HasSynDefnTokraRegion for SubmoduleSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        todo!()
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        todo!()
    }
}

impl HasSynDefnTokraRegion for MajorItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => syn_node_path.syn_defn_tokra_region(db),
            MajorItemSynNodePath::Type(syn_node_path) => syn_node_path.syn_defn_tokra_region(db),
            MajorItemSynNodePath::Fugitive(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region(db)
            }
        }
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            MajorItemSynNodePath::Type(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            MajorItemSynNodePath::Fugitive(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
        }
    }
}

struct SynDefnTokraRegionBuilder<'a> {
    db: &'a dyn EntitySynTreeDb,
    ast_sheet: &'a AstSheet,
    syn_defn_ast_arena: DefnAstArena,
}

impl<'a> SynDefnTokraRegionBuilder<'a> {
    fn new(module_path: ModulePath, db: &'a dyn EntitySynTreeDb) -> Self {
        Self {
            db,
            ast_sheet: module_path.ast_sheet(db).expect("modules should be valid"),
            syn_defn_ast_arena: Default::default(),
        }
    }

    fn build(self, ast_idx: AstIdx) -> Option<(DefnTokraRegion, SynDefnTokraRegionSourceMap)> {
        let children = match self.ast_sheet[ast_idx] {
            Ast::Identifiable { block, .. } => block.children()?,
            _ => unreachable!(),
        };
        todo!()
    }

    fn finish(self) -> Option<(DefnTokraRegion, SynDefnTokraRegionSourceMap)> {
        todo!()
    }
}

fn build_defn_tokra_region(
    module_path: ModulePath,
    ast_idx: AstIdx,
    db: &dyn EntitySynTreeDb,
) -> Option<(DefnTokraRegion, SynDefnTokraRegionSourceMap)> {
    let mut builder = SynDefnTokraRegionBuilder::new(module_path, db);
    builder.build(ast_idx)
}

impl HasSynDefnTokraRegion for TraitSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        None
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        None
    }
}

impl HasSynDefnTokraRegion for TypeSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        None
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        None
    }
}

impl HasSynDefnTokraRegion for FugitiveSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        fugitive_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        fugitive_defn_tokra_region_with_source_map(db, self).map(|v| v.1)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: FugitiveSynNodePath,
) -> Option<DefnTokraRegion> {
    fugitive_defn_tokra_region_with_source_map(db, syn_node_path).map(|v| v.0)
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: FugitiveSynNodePath,
) -> Option<(DefnTokraRegion, SynDefnTokraRegionSourceMap)> {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

impl HasSynDefnTokraRegion for TypeVariantSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        None
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        None
    }
}

impl HasSynDefnTokraRegion for ImplBlockSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        match self {
            ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region(db)
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region(db)
            }
            ImplBlockSynNodePath::IllFormedImplBlock(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region(db)
            }
        }
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        match self {
            ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            ImplBlockSynNodePath::IllFormedImplBlock(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
        }
    }
}

impl HasSynDefnTokraRegion for TypeImplBlockSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        None
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        None
    }
}

impl HasSynDefnTokraRegion for TraitForTypeImplBlockSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        None
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        None
    }
}

impl HasSynDefnTokraRegion for IllFormedImplBlockSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        None
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        None
    }
}

impl HasSynDefnTokraRegion for AssociatedItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region(db)
            }
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region(db)
            }
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region(db)
            }
            AssociatedItemSynNodePath::IllFormedItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region(db)
            }
        }
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
            AssociatedItemSynNodePath::IllFormedItem(syn_node_path) => {
                syn_node_path.syn_defn_tokra_region_source_map(db)
            }
        }
    }
}

impl HasSynDefnTokraRegion for TypeItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        ty_item_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        ty_item_defn_tokra_region_with_source_map(db, self).map(|v| v.1)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeItemSynNodePath,
) -> Option<(DefnTokraRegion, SynDefnTokraRegionSourceMap)> {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeItemSynNodePath,
) -> Option<DefnTokraRegion> {
    ty_item_defn_tokra_region_with_source_map(db, syn_node_path).map(|v| v.0)
}

impl HasSynDefnTokraRegion for TraitItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        trai_item_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        trai_item_defn_tokra_region_with_source_map(db, self).map(|v| v.1)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitItemSynNodePath,
) -> Option<(DefnTokraRegion, SynDefnTokraRegionSourceMap)> {
    todo!()
    // build_defn_tokra_region(
    //     syn_node_path.module_path(db),
    //     syn_node_path.node(db).ast_idx(db),
    //     db,
    // )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_item_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitItemSynNodePath,
) -> Option<DefnTokraRegion> {
    trai_item_defn_tokra_region_with_source_map(db, syn_node_path).map(|v| v.0)
}

impl HasSynDefnTokraRegion for TraitForTypeItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        trai_for_ty_item_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        trai_for_ty_item_defn_tokra_region_with_source_map(db, self).map(|v| v.1)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> Option<(DefnTokraRegion, SynDefnTokraRegionSourceMap)> {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> Option<DefnTokraRegion> {
    trai_for_ty_item_defn_tokra_region_with_source_map(db, syn_node_path).map(|v| v.0)
}

impl HasSynDefnTokraRegion for IllFormedItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        ill_formed_item_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        ill_formed_item_defn_tokra_region_with_source_map(db, self).map(|v| v.1)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedItemSynNodePath,
) -> Option<(DefnTokraRegion, SynDefnTokraRegionSourceMap)> {
    todo!()
    // build_defn_tokra_region(
    //     syn_node_path.module_path(db),
    //     syn_node_path.node(db).ast_idx(db),
    //     db,
    // )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_item_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedItemSynNodePath,
) -> Option<DefnTokraRegion> {
    ill_formed_item_defn_tokra_region_with_source_map(db, syn_node_path).map(|v| v.0)
}

impl HasSynDefnTokraRegion for DecrSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> Option<DefnTokraRegion> {
        None
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        None
    }
}
