use super::*;
use husky_decl_ast::DeclAst;
use husky_token::{TokenGroupIdx, TokenIdxRange, TokenSheetData};

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

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct DeclTokraRegion {
    #[return_ref]
    _tokens_data: Vec<TokenData>,
    pub saved_regional_token_stream_state: Option<RegionalTokenStreamState>,
    pub ast: DeclAst,
}

impl DeclTokraRegion {
    pub fn data<'a>(self, db: &'a dyn EntitySynTreeDb) -> DeclTokraRegionData<'a> {
        DeclTokraRegionData {
            tokens_data: self._tokens_data(db),
            saved_regional_token_stream_state: self.saved_regional_token_stream_state(db),
        }
    }

    pub fn tokens_data<'a>(self, db: &'a dyn EntitySynTreeDb) -> RegionalTokensData<'a> {
        RegionalTokensData::new(self._tokens_data(db))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeclTokraRegionData<'a> {
    saved_regional_token_stream_state: Option<RegionalTokenStreamState>,
    tokens_data: &'a [TokenData],
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

fn build_decl_tokra_region(
    module_path: ModulePath,
    ast_idx: AstIdx,
    db: &dyn EntitySynTreeDb,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    let token_sheet_data = db
        .token_sheet_data(module_path)
        .expect("all modules should be valid");
    let ast_sheet = db
        .ast_sheet(module_path)
        .expect("all modules should be valid");
    let (token_group_idx, ast, saved_regional_stream_state) = match ast_sheet[ast_idx] {
        Ast::Attr {
            token_group_idx,
            ident,
        } => (token_group_idx, DeclAst::Attr, None),
        Ast::Identifiable {
            token_group_idx,
            ref visibility_expr,
            item_kind,
            ident_token,
            is_generic,
            saved_stream_state,
            block,
        } => (
            token_group_idx,
            DeclAst::Identifiable {},
            Some(saved_stream_state),
        ),
        Ast::TypeVariant {
            token_group_idx,
            variant_path,
            vertical_token,
            ident_token,
            saved_stream_state,
        } => (
            token_group_idx,
            DeclAst::TypeVariant,
            Some(saved_stream_state),
        ),
        Ast::ImplBlock {
            token_group_idx,
            items,
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

pub trait HasDeclTokraRegion:
    for<'a> HasModulePath<dyn EntitySynTreeDb + 'a> + Copy + Into<ItemSynNodePath>
{
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion;
    // use this only when necessary
    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap;

    fn decl_ast_idx(self, db: &dyn EntitySynTreeDb) -> AstIdx {
        self.decl_tokra_region_source_map(db).ast_idx()
    }

    fn decl_regional_token_idx_base(self, db: &dyn EntitySynTreeDb) -> RegionalTokenIdxBase {
        self.decl_tokra_region_source_map(db)
            .regional_token_idx_base
    }

    fn decl_tokra_region_token_idx_range(self, db: &dyn EntitySynTreeDb) -> TokenIdxRange {
        let decl_tokra_region = self.decl_tokra_region(db);
        let decl_tokra_region_source_map = self.decl_tokra_region_source_map(db);
        let start = decl_tokra_region_source_map
            .regional_token_idx_base
            .index_base();
        let end = start + decl_tokra_region.tokens_data(db).len();
        unsafe {
            TokenIdxRange::new(
                TokenIdx::from_usize_index_ext(start),
                TokenIdx::from_usize_index_ext(end),
            )
        }
    }
}

impl HasDeclTokraRegion for ItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::TypeVariant(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::AssociatedItem(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::Attr(syn_node_path) => syn_node_path.decl_tokra_region(db),
        }
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
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
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        submodule_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        submodule_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn submodule_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: SubmoduleSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn submodule_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: SubmoduleSynNodePath,
) -> DeclTokraRegion {
    submodule_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for MajorItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => syn_node_path.decl_tokra_region(db),
            MajorItemSynNodePath::Type(syn_node_path) => syn_node_path.decl_tokra_region(db),
            MajorItemSynNodePath::Fugitive(syn_node_path) => syn_node_path.decl_tokra_region(db),
        }
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
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
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        trai_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        trai_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitSynNodePath,
) -> DeclTokraRegion {
    trai_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for TypeSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        ty_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        ty_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeSynNodePath,
) -> DeclTokraRegion {
    ty_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for FugitiveSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        fugitive_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        fugitive_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: FugitiveSynNodePath,
) -> DeclTokraRegion {
    fugitive_decl_tokra_region_with_source_map(db, syn_node_path).0
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: FugitiveSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

impl HasDeclTokraRegion for TypeVariantSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        ty_variant_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        ty_variant_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_variant_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeVariantSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_variant_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeVariantSynNodePath,
) -> DeclTokraRegion {
    ty_variant_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for ImplBlockSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
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

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
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
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        ty_impl_block_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        ty_impl_block_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_impl_block_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeImplBlockSynNodePath,
) -> DeclTokraRegion {
    ty_impl_block_decl_tokra_region_with_source_map(db, syn_node_path).0
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_impl_block_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeImplBlockSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

impl HasDeclTokraRegion for TraitForTypeImplBlockSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        trai_for_ty_impl_block_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        trai_for_ty_impl_block_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_impl_block_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_impl_block_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> DeclTokraRegion {
    trai_for_ty_impl_block_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for IllFormedImplBlockSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        ill_formed_impl_block_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        ill_formed_impl_block_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_impl_block_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_impl_block_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> DeclTokraRegion {
    ill_formed_impl_block_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for AssociatedItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
            AssociatedItemSynNodePath::IllFormedItem(syn_node_path) => {
                syn_node_path.decl_tokra_region(db)
            }
        }
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
            AssociatedItemSynNodePath::IllFormedItem(syn_node_path) => {
                syn_node_path.decl_tokra_region_source_map(db)
            }
        }
    }
}

impl HasDeclTokraRegion for TypeItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        ty_item_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        ty_item_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeItemSynNodePath,
) -> DeclTokraRegion {
    ty_item_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for TraitItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        trai_item_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        trai_item_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_item_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    todo!()
    // build_decl_tokra_region(
    //     syn_node_path.module_path(db),
    //     syn_node_path.node(db).ast_idx(db),
    //     db,
    // )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_item_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitItemSynNodePath,
) -> DeclTokraRegion {
    trai_item_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for TraitForTypeItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        trai_for_ty_item_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        trai_for_ty_item_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> DeclTokraRegion {
    trai_for_ty_item_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for IllFormedItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        ill_formed_item_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        ill_formed_item_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_item_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    todo!()
    // build_decl_tokra_region(
    //     syn_node_path.module_path(db),
    //     syn_node_path.node(db).ast_idx(db),
    //     db,
    // )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_item_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedItemSynNodePath,
) -> DeclTokraRegion {
    ill_formed_item_decl_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasDeclTokraRegion for AttrSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        attr_decl_tokra_region(db, self)
    }

    fn decl_tokra_region_source_map(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegionSourceMap {
        attr_decl_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn attr_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: AttrSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.syn_node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn attr_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: AttrSynNodePath,
) -> DeclTokraRegion {
    attr_decl_tokra_region_with_source_map(db, syn_node_path).0
}
