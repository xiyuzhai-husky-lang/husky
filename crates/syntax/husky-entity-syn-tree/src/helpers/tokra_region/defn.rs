use super::*;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct DefnTokraRegion {
    #[id]
    syn_node_path: ItemSynNodePath,
    #[return_ref]
    tokens: Vec<Token>,
}

impl DefnTokraRegion {
    pub fn data<'a>(self, db: &'a dyn EntitySynTreeDb) -> DefnTokraRegionData<'a> {
        DefnTokraRegionData {
            tokens: self.tokens(db),
        }
    }
}

pub struct DefnTokraRegionData<'a> {
    tokens: &'a [Token],
}

impl<'a> std::ops::Index<RegionalTokenIdx> for DefnTokraRegionData<'a> {
    type Output = Token;

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

pub enum DefnAst {}

pub type DefnAstArena = Arena<DefnAst>;
pub type DefnAstArenaIdx = ArenaIdx<DefnAst>;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct DefnTokraRegionSourceMap {
    token_region_base: TokenRegionBase,
}

pub trait HasDefnTokraRegion: for<'a> HasModulePath<dyn EntitySynTreeDb + 'a> + Copy {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion;
    // use this only when necessary
    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap;
}

impl HasDefnTokraRegion for ItemSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => syn_node_path.defn_tokra_region(db),
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.defn_tokra_region(db),
            ItemSynNodePath::TypeVariant(syn_node_path) => syn_node_path.defn_tokra_region(db),
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.defn_tokra_region(db),
            ItemSynNodePath::AssociatedItem(syn_node_path) => syn_node_path.defn_tokra_region(db),
            ItemSynNodePath::Decr(syn_node_path) => syn_node_path.defn_tokra_region(db),
        }
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::MajorItem(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::TypeVariant(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::ImplBlock(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::AssociatedItem(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            ItemSynNodePath::Decr(syn_node_path) => syn_node_path.defn_tokra_region_source_map(db),
        }
    }
}

impl HasDefnTokraRegion for SubmoduleSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        todo!()
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        todo!()
    }
}

impl HasDefnTokraRegion for MajorItemSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => syn_node_path.defn_tokra_region(db),
            MajorItemSynNodePath::Type(syn_node_path) => syn_node_path.defn_tokra_region(db),
            MajorItemSynNodePath::Fugitive(syn_node_path) => syn_node_path.defn_tokra_region(db),
        }
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            MajorItemSynNodePath::Type(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            MajorItemSynNodePath::Fugitive(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
        }
    }
}

fn build_defn_tokra_region(
    module_path: ModulePath,
    ast_idx: AstIdx,
    db: &dyn EntitySynTreeDb,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    todo!()
}

impl HasDefnTokraRegion for TraitSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        trai_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        trai_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitSynNodePath,
) -> DefnTokraRegion {
    trai_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDefnTokraRegion for TypeSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        ty_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        ty_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeSynNodePath,
) -> DefnTokraRegion {
    ty_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDefnTokraRegion for FugitiveSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        fugitive_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        fugitive_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: FugitiveSynNodePath,
) -> DefnTokraRegion {
    fugitive_defn_tokra_region_with_source_map(db, syn_node_path).0
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: FugitiveSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

impl HasDefnTokraRegion for TypeVariantSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        ty_variant_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        ty_variant_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_variant_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeVariantSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_variant_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeVariantSynNodePath,
) -> DefnTokraRegion {
    ty_variant_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDefnTokraRegion for ImplBlockSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        match self {
            ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                syn_node_path.defn_tokra_region(db)
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(syn_node_path) => {
                syn_node_path.defn_tokra_region(db)
            }
            ImplBlockSynNodePath::IllFormedImplBlock(syn_node_path) => {
                syn_node_path.defn_tokra_region(db)
            }
        }
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        match self {
            ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            ImplBlockSynNodePath::IllFormedImplBlock(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
        }
    }
}

impl HasDefnTokraRegion for TypeImplBlockSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        ty_impl_block_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        ty_impl_block_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_impl_block_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeImplBlockSynNodePath,
) -> DefnTokraRegion {
    ty_impl_block_defn_tokra_region_with_source_map(db, syn_node_path).0
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_impl_block_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeImplBlockSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

impl HasDefnTokraRegion for TraitForTypeImplBlockSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        trai_for_ty_impl_block_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        trai_for_ty_impl_block_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_impl_block_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_impl_block_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> DefnTokraRegion {
    trai_for_ty_impl_block_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDefnTokraRegion for IllFormedImplBlockSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        ill_formed_impl_block_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        ill_formed_impl_block_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_impl_block_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_impl_block_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> DefnTokraRegion {
    ill_formed_impl_block_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDefnTokraRegion for AssociatedItemSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                syn_node_path.defn_tokra_region(db)
            }
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                syn_node_path.defn_tokra_region(db)
            }
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.defn_tokra_region(db)
            }
            AssociatedItemSynNodePath::IllFormedItem(syn_node_path) => {
                syn_node_path.defn_tokra_region(db)
            }
        }
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
            AssociatedItemSynNodePath::IllFormedItem(syn_node_path) => {
                syn_node_path.defn_tokra_region_source_map(db)
            }
        }
    }
}

impl HasDefnTokraRegion for TypeItemSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        ty_item_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        ty_item_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeItemSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeItemSynNodePath,
) -> DefnTokraRegion {
    ty_item_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDefnTokraRegion for TraitItemSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        trai_item_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        trai_item_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitItemSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
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
) -> DefnTokraRegion {
    trai_item_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDefnTokraRegion for TraitForTypeItemSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        trai_for_ty_item_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        trai_for_ty_item_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> DefnTokraRegion {
    trai_for_ty_item_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDefnTokraRegion for IllFormedItemSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        ill_formed_item_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        ill_formed_item_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedItemSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
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
) -> DefnTokraRegion {
    ill_formed_item_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDefnTokraRegion for DecrSynNodePath {
    fn defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegion {
        decr_defn_tokra_region(db, self)
    }

    fn defn_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DefnTokraRegionSourceMap {
        decr_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn decr_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: DecrSynNodePath,
) -> (DefnTokraRegion, DefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn decr_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: DecrSynNodePath,
) -> DefnTokraRegion {
    decr_defn_tokra_region_with_source_map(db, syn_node_path).0
}
