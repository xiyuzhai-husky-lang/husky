use idx_arena::ArenaRef;

use super::*;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct SynDefnTokraRegion {
    #[return_ref]
    tokens: Vec<Token>,
    #[return_ref]
    ast_arena: SynDefnAstArena,
}

impl SynDefnTokraRegion {
    pub fn data<'a>(self, db: &'a dyn EntitySynTreeDb) -> SynDefnTokraRegionData<'a> {
        SynDefnTokraRegionData {
            tokens: self.tokens(db),
            ast_arena: self.ast_arena(db).to_ref(),
        }
    }
}

pub struct SynDefnTokraRegionData<'a> {
    tokens: &'a [Token],
    ast_arena: DefnAstArenaRef<'a>,
}

impl<'a> std::ops::Index<RegionalTokenIdx> for SynDefnTokraRegionData<'a> {
    type Output = Token;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[idx.index()]
    }
}

pub(super) fn defn_token_region(
    syn_node_path: ItemSynNodePath,
    db: &dyn EntitySynTreeDb,
) -> Option<SynDefnTokraRegion> {
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

pub type SynDefnAstArena = Arena<DefnAst>;
pub type DefnAstArenaRef<'a> = ArenaRef<'a, DefnAst>;
pub type DefnAstArenaIdx = ArenaIdx<DefnAst>;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct SynDefnTokraRegionSourceMap {
    token_region_base: TokenRegionBase,
}

pub trait HasSynDefnTokraRegion: for<'a> HasModulePath<dyn EntitySynTreeDb + 'a> + Copy {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion;
    // use this only when necessary
    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap;
}

impl HasSynDefnTokraRegion for ItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
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
    ) -> SynDefnTokraRegionSourceMap {
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
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        todo!()
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        todo!()
    }
}

impl HasSynDefnTokraRegion for MajorItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
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
    ) -> SynDefnTokraRegionSourceMap {
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
    syn_defn_ast_arena: SynDefnAstArena,
}

impl<'a> SynDefnTokraRegionBuilder<'a> {
    fn new(module_path: ModulePath, db: &'a dyn EntitySynTreeDb) -> Self {
        Self {
            db,
            ast_sheet: module_path.ast_sheet(db).expect("modules should be valid"),
            syn_defn_ast_arena: Default::default(),
        }
    }

    fn build(self, ast_idx: AstIdx) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
        match self.ast_sheet[ast_idx] {
            Ast::Err {
                token_group_idx,
                ref error,
            } => todo!(),
            Ast::Use {
                token_group_idx,
                ref visibility_expr,
                state_after_visibility_expr,
            } => todo!(),
            Ast::Sorc { token_group_idx } => todo!(),
            Ast::Decr {
                token_group_idx,
                ident,
            } => todo!(),
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => todo!(),
            Ast::IfElseStmts {
                if_branch,
                elif_branches,
                else_branch,
            } => todo!(),
            Ast::MatchStmts {
                token_group_idx,
                pattern_stmt,
                case_stmts,
            } => todo!(),
            Ast::Identifiable {
                token_group_idx,
                ref visibility_expr,
                item_kind,
                ident_token,
                is_generic,
                saved_stream_state,
                block,
            } => todo!(),
            Ast::TypeVariant {
                token_group_idx,
                variant_path,
                vertical_token,
                ident_token,
                state_after,
            } => todo!(),
            Ast::ImplBlock {
                token_group_idx,
                items,
            } => todo!(),
        }
    }

    fn finish(self) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
        todo!()
    }
}

fn build_defn_tokra_region(
    module_path: ModulePath,
    ast_idx: AstIdx,
    db: &dyn EntitySynTreeDb,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
    let mut ast_arena: SynDefnAstArena = Default::default();
    todo!()
}

impl HasSynDefnTokraRegion for TraitSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        trai_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        trai_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    trai_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasSynDefnTokraRegion for TypeSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        ty_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        ty_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    ty_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasSynDefnTokraRegion for FugitiveSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        fugitive_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        fugitive_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: FugitiveSynNodePath,
) -> SynDefnTokraRegion {
    fugitive_defn_tokra_region_with_source_map(db, syn_node_path).0
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn fugitive_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: FugitiveSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

impl HasSynDefnTokraRegion for TypeVariantSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        ty_variant_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        ty_variant_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_variant_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeVariantSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    ty_variant_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasSynDefnTokraRegion for ImplBlockSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
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
    ) -> SynDefnTokraRegionSourceMap {
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
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        ty_impl_block_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        ty_impl_block_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_impl_block_defn_tokra_region(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeImplBlockSynNodePath,
) -> SynDefnTokraRegion {
    ty_impl_block_defn_tokra_region_with_source_map(db, syn_node_path).0
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_impl_block_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeImplBlockSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
    build_defn_tokra_region(
        syn_node_path.module_path(db),
        syn_node_path.node(db).ast_idx(db),
        db,
    )
}

impl HasSynDefnTokraRegion for TraitForTypeImplBlockSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        trai_for_ty_impl_block_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        trai_for_ty_impl_block_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_impl_block_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    trai_for_ty_impl_block_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasSynDefnTokraRegion for IllFormedImplBlockSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        ill_formed_impl_block_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        ill_formed_impl_block_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_impl_block_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    ill_formed_impl_block_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasSynDefnTokraRegion for AssociatedItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
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
    ) -> SynDefnTokraRegionSourceMap {
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
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        ty_item_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        ty_item_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeItemSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    ty_item_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasSynDefnTokraRegion for TraitItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        trai_item_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        trai_item_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitItemSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    trai_item_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasSynDefnTokraRegion for TraitForTypeItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        trai_for_ty_item_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        trai_for_ty_item_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_for_ty_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    trai_for_ty_item_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasSynDefnTokraRegion for IllFormedItemSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        ill_formed_item_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        ill_formed_item_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ill_formed_item_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedItemSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    ill_formed_item_defn_tokra_region_with_source_map(db, syn_node_path).0
}

impl HasSynDefnTokraRegion for DecrSynNodePath {
    fn syn_defn_tokra_region(self, db: &dyn EntitySynTreeDb) -> SynDefnTokraRegion {
        decr_defn_tokra_region(db, self)
    }

    fn syn_defn_tokra_region_source_map(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> SynDefnTokraRegionSourceMap {
        decr_defn_tokra_region_with_source_map(db, self).1
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn decr_defn_tokra_region_with_source_map(
    db: &dyn EntitySynTreeDb,
    syn_node_path: DecrSynNodePath,
) -> (SynDefnTokraRegion, SynDefnTokraRegionSourceMap) {
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
) -> SynDefnTokraRegion {
    decr_defn_tokra_region_with_source_map(db, syn_node_path).0
}
