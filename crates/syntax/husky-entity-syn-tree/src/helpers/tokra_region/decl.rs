use super::*;
use husky_decl_ast::DeclAst;
use husky_token::{TokenDb, TokenIdxRange};

///
#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct DeclTokraRegion {
    #[return_ref]
    tokens_data: Vec<TokenData>,
    pub saved_regional_token_stream_state: Option<RegionalTokenStreamState>,
    pub ast: DeclAst,
}

#[derive(Debug, Clone, Copy)]
pub struct DeclTokraRegionData<'a> {
    saved_regional_token_stream_state: Option<RegionalTokenStreamState>,
    tokens_data: &'a [TokenData],
}

impl DeclTokraRegion {
    pub fn data<'a>(self, db: &'a ::salsa::Db) -> DeclTokraRegionData<'a> {
        DeclTokraRegionData {
            tokens_data: self.tokens_data(db),
            saved_regional_token_stream_state: self.saved_regional_token_stream_state(db),
        }
    }

    pub fn regional_tokens_data<'a>(self, db: &'a ::salsa::Db) -> RegionalTokensData<'a> {
        RegionalTokensData::new(self.tokens_data(db))
    }
}

impl<'a> DeclTokraRegionData<'a> {
    pub fn regional_token_stream(self) -> RegionalTokenStream<'a> {
        RegionalTokenStream::new_decl_regional_token_stream(
            self.tokens_data,
            self.saved_regional_token_stream_state,
        )
    }
}

impl<'a> std::ops::Index<RegionalTokenIdx> for DeclTokraRegionData<'a> {
    type Output = TokenData;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens_data[idx.index()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclTokraRegionSourceMap {
    regional_token_idx_base: RegionalTokenIdxBase,
    ast_idx: AstIdx,
}

impl DeclTokraRegionSourceMap {
    pub fn regional_token_idx_base(&self) -> RegionalTokenIdxBase {
        self.regional_token_idx_base
    }

    pub fn ast_idx(&self) -> ArenaIdx<Ast> {
        self.ast_idx
    }
}

fn build_decl_tokra_region(
    module_path: ModulePath,
    ast_idx: AstIdx,
    db: &::salsa::Db,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    let token_sheet_data = db.token_sheet_data(module_path);
    let ast_sheet = db.ast_sheet(module_path);
    let (token_group_idx, ast, saved_regional_stream_state) = match ast_sheet[ast_idx] {
        Ast::Attr {
            token_group_idx,
            ident: _,
        } => (token_group_idx, DeclAst::Attr, None),
        Ast::Identifiable {
            token_group_idx,
            visibility_expr: _,
            item_kind: _,
            ident_token: _,
            is_generic: _,
            saved_stream_state,
            block: _,
        } => (
            token_group_idx,
            DeclAst::Identifiable {},
            Some(saved_stream_state),
        ),
        Ast::TypeVariant {
            token_group_idx,
            variant_path: _,
            vertical_token: _,
            ident_token: _,
            saved_stream_state,
        } => (
            token_group_idx,
            DeclAst::TypeVariant,
            Some(saved_stream_state),
        ),
        Ast::ImplBlock {
            token_group_idx,
            items: _,
        } => (token_group_idx, DeclAst::ImplBlock, None),
        _ => unreachable!(),
    };
    let tokens = token_sheet_data[token_group_idx].to_vec();
    let regional_token_idx_base =
        RegionalTokenIdxBase::new(token_sheet_data.token_group_start(token_group_idx));
    let saved_regional_stream_state = saved_regional_stream_state.map(|token_stream_state| {
        RegionalTokenStreamState::from_token_stream_state(
            token_stream_state,
            regional_token_idx_base,
        )
    });
    let decl_tokra_region =
        DeclTokraRegion::new_inner(db, tokens, saved_regional_stream_state, ast);
    let decl_tokra_region_source_map = DeclTokraRegionSourceMap {
        regional_token_idx_base,
        ast_idx,
    };
    (decl_tokra_region, decl_tokra_region_source_map)
}

pub trait HasDeclTokraRegion: Copy + Into<ItemSynNodePath> {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion;
    // use this only when necessary
    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap;

    fn decl_ast_idx(self, db: &::salsa::Db) -> AstIdx {
        self.decl_tokra_region_source_map(db).ast_idx()
    }

    fn decl_regional_token_idx_base(self, db: &::salsa::Db) -> RegionalTokenIdxBase {
        self.decl_tokra_region_source_map(db)
            .regional_token_idx_base
    }

    fn decl_tokra_region_token_idx_range(self, db: &::salsa::Db) -> TokenIdxRange {
        let decl_tokra_region = self.decl_tokra_region(db);
        let decl_tokra_region_source_map = self.decl_tokra_region_source_map(db);
        let start = decl_tokra_region_source_map
            .regional_token_idx_base
            .index_base();
        let end = start + decl_tokra_region.regional_tokens_data(db).len();
        unsafe {
            TokenIdxRange::new(
                TokenIdx::from_usize_index_ext(start),
                TokenIdx::from_usize_index_ext(end),
            )
        }
    }
}

impl HasDeclTokraRegion for ItemSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::TypeVariant(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::AssociatedItem(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::Attr(syn_node_path) => syn_node_path.decl_tokra_region(db),
        }
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            ItemSynNodePath::MajorItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            ItemSynNodePath::TypeVariant(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            ItemSynNodePath::ImplBlock(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            ItemSynNodePath::AssociatedItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            ItemSynNodePath::Attr(syn_node_path) => syn_node_path.decl_tokra_region_source_map(db),
        }
    }
}

impl HasDeclTokraRegion for SubmoduleSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        submodule_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        submodule_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn submodule_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: SubmoduleSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node_data(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn submodule_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: SubmoduleSynNodePath,
) -> DeclTokraRegion {
    submodule_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for MajorItemSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => syn_node_path.decl_tokra_region(db),
            MajorItemSynNodePath::Type(syn_node_path) => syn_node_path.decl_tokra_region(db),
            MajorItemSynNodePath::Fugitive(syn_node_path) => syn_node_path.decl_tokra_region(db),
        }
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            MajorItemSynNodePath::Type(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            MajorItemSynNodePath::Fugitive(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
        }
    }
}

impl HasDeclTokraRegion for TraitSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        trai_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        trai_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: TraitSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_decl_tokra_region(db: &::salsa::Db, syn_node_path: TraitSynNodePath) -> DeclTokraRegion {
    trai_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for TypeSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        ty_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        ty_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: TypeSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_decl_tokra_region(db: &::salsa::Db, syn_node_path: TypeSynNodePath) -> DeclTokraRegion {
    ty_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for FugitiveSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        fugitive_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        fugitive_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: FugitiveSynNodePath,
) -> DeclTokraRegion {
    fugitive_decl_tokra_region_with_source_map(db, syn_node_path).0
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: FugitiveSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

impl HasDeclTokraRegion for TypeVariantSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        ty_variant_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        ty_variant_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_variant_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: TypeVariantSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_variant_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: TypeVariantSynNodePath,
) -> DeclTokraRegion {
    ty_variant_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for ImplBlockSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        match self {
            ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
            ImplBlockSynNodePath::IllFormedImplBlock(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
        }
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        match self {
            ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            ImplBlockSynNodePath::IllFormedImplBlock(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
        }
    }
}

impl HasDeclTokraRegion for TypeImplBlockSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        ty_impl_block_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        ty_impl_block_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_impl_block_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: TypeImplBlockSynNodePath,
) -> DeclTokraRegion {
    ty_impl_block_decl_tokra_region_with_source_map(db, syn_node_path).0
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_impl_block_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: TypeImplBlockSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

impl HasDeclTokraRegion for TraitForTypeImplBlockSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        trai_for_ty_impl_block_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        trai_for_ty_impl_block_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_impl_block_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_impl_block_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> DeclTokraRegion {
    trai_for_ty_impl_block_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for IllFormedImplBlockSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        ill_formed_impl_block_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        ill_formed_impl_block_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_impl_block_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(),
        // todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_impl_block_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> DeclTokraRegion {
    ill_formed_impl_block_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for AssociatedItemSynNodeDataPath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        match self {
            AssociatedItemSynNodeDataPath::TypeItem(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
            AssociatedItemSynNodeDataPath::TraitItem(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
            AssociatedItemSynNodeDataPath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
            AssociatedItemSynNodeDataPath::IllFormedItem(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
        }
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        match self {
            AssociatedItemSynNodeDataPath::TypeItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            AssociatedItemSynNodeDataPath::TraitItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            AssociatedItemSynNodeDataPath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            AssociatedItemSynNodeDataPath::IllFormedItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
        }
    }
}

impl HasDeclTokraRegion for TypeItemSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        ty_item_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        ty_item_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: TypeItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: TypeItemSynNodePath,
) -> DeclTokraRegion {
    ty_item_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for TraitItemSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        trai_item_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        trai_item_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_item_decl_tokra_region_with_source_map(
    _db: &::salsa::Db,
    _syn_node_path: TraitItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    todo!()
    // build_decl_tokra_region(
    //     todo!(), /* syn_node_path.module_path(db),*/
    //     syn_node_path.node(db).ast_idx(db),
    //     db,
    // )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_item_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: TraitItemSynNodePath,
) -> DeclTokraRegion {
    trai_item_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for TraitForTypeItemSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        trai_for_ty_item_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        trai_for_ty_item_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> DeclTokraRegion {
    trai_for_ty_item_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for IllFormedItemSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        ill_formed_item_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        ill_formed_item_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_item_decl_tokra_region_with_source_map(
    _db: &::salsa::Db,
    _syn_node_path: IllFormedItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    todo!()
    // build_decl_tokra_region(
    //     todo!(), /* syn_node_path.module_path(db),*/
    //     syn_node_path.node(db).ast_idx(db),
    //     db,
    // )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_item_decl_tokra_region(
    db: &::salsa::Db,
    syn_node_path: IllFormedItemSynNodePath,
) -> DeclTokraRegion {
    ill_formed_item_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for AttrSynNodePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        attr_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        attr_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn attr_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    syn_node_path: AttrSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        todo!(), /* syn_node_path.module_path(db),*/
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn attr_decl_tokra_region(db: &::salsa::Db, syn_node_path: AttrSynNodePath) -> DeclTokraRegion {
    attr_decl_tokra_region_with_source_map(db, syn_node_path).0
}
