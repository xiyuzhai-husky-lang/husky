use super::*;
use husky_token::{TokenGroupIdx, TokenSheetData};

pub enum DeclAst {
    Identifiable,
    TypeVariant,
    ImplBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclTokraRegionSourceMap {
    token_region_base: TokenRegionBase,
    ast_idx: AstIdx,
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct DeclTokraRegion {
    #[return_ref]
    tokens: Vec<Token>,
}

impl DeclTokraRegion {
    pub fn data<'a>(self, db: &'a dyn EntitySynTreeDb) -> DeclTokraRegionData<'a> {
        DeclTokraRegionData {
            tokens: self.tokens(db),
        }
    }
}

pub struct DeclTokraRegionData<'a> {
    tokens: &'a [Token],
}

impl<'a> std::ops::Index<RegionalTokenIdx> for DeclTokraRegionData<'a> {
    type Output = Token;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[idx.index()]
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
    let (token_group_idx,) = match ast_sheet[ast_idx] {
        Ast::Decr {
            token_group_idx,
            ident,
        } => (token_group_idx,),
        Ast::Identifiable {
            token_group_idx,
            ref visibility_expr,
            item_kind,
            ident_token,
            is_generic,
            saved_stream_state,
            block,
        } => (token_group_idx,),
        Ast::TypeVariant {
            token_group_idx,
            variant_path,
            vertical_token,
            ident_token,
            state_after,
        } => (token_group_idx,),
        Ast::ImplBlock {
            token_group_idx,
            items,
        } => (token_group_idx,),
        _ => unreachable!(),
    };
    let tokens = token_sheet_data[token_group_idx].to_vec();
    let token_region_base =
        TokenRegionBase::new(token_sheet_data.token_group_base(token_group_idx));
    let decl_tokra_region = DeclTokraRegion::new_inner(db, tokens);
    let decl_tokra_region_source_map = DeclTokraRegionSourceMap {
        token_region_base,
        ast_idx,
    };
    (decl_tokra_region, decl_tokra_region_source_map)
}

pub trait HasDeclTokraRegion: for<'a> HasModulePath<dyn EntitySynTreeDb + 'a> + Copy {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion;
}

impl HasDeclTokraRegion for ItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        match self {
            ItemSynNodePath::Submodule(_) => todo!(),
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.decl_tokra_region(db),
            ItemSynNodePath::TypeVariant(_) => todo!(),
            ItemSynNodePath::ImplBlock(_) => todo!(),
            ItemSynNodePath::AssociatedItem(_) => todo!(),
            ItemSynNodePath::Decr(_) => todo!(),
        }
    }
}

impl HasDeclTokraRegion for MajorItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        match self {
            MajorItemSynNodePath::Trait(_) => todo!(),
            MajorItemSynNodePath::Type(_) => todo!(),
            MajorItemSynNodePath::Fugitive(_) => todo!(),
        }
    }
}

impl HasDeclTokraRegion for TraitSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        trai_decl_tokra_region(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
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
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
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
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

impl HasDeclTokraRegion for TypeVariantSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        ty_variant_decl_tokra_region(db, self)
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
            ImplBlockSynNodePath::TypeImplBlock(_) => todo!(),
            ImplBlockSynNodePath::TraitForTypeImplBlock(_) => todo!(),
            ImplBlockSynNodePath::IllFormedImplBlock(_) => todo!(),
        }
    }
}

impl HasDeclTokraRegion for TypeImplBlockSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        ty_impl_block_decl_tokra_region(db, self)
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
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

impl HasDeclTokraRegion for TraitForTypeImplBlockSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        trai_for_ty_impl_block_decl_tokra_region(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_impl_block_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
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
        todo!()
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_impl_block_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
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
            AssociatedItemSynNodePath::TypeItem(_) => todo!(),
            AssociatedItemSynNodePath::TraitItem(_) => todo!(),
            AssociatedItemSynNodePath::TraitForTypeItem(_) => todo!(),
            AssociatedItemSynNodePath::IllFormedItem(_) => todo!(),
        }
    }
}

impl HasDeclTokraRegion for TypeItemSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        ty_item_decl_tokra_region(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
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
        todo!()
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
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
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

impl HasDeclTokraRegion for DecrSynNodePath {
    fn decl_tokra_region(self, db: &dyn EntitySynTreeDb) -> DeclTokraRegion {
        decr_decl_tokra_region(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn decr_decl_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: DecrSynNodePath,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn decr_decl_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: DecrSynNodePath,
) -> DeclTokraRegion {
    decr_decl_tokra_region_with_source_map(db, syn_node_path).0
}
