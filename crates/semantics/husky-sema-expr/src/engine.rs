mod expect;
mod expr_term;
mod symbol;
#[macro_use]
mod utils;
mod branch_ty_merger;
mod pattern_ty;

pub(crate) use self::branch_ty_merger::*;
pub(crate) use self::utils::*;

use self::symbol::*;
use crate::*;
use husky_entity_tree::helpers::TraitInUseItemsTable;
use husky_eth_signature::HasEthTemplate;
use husky_eth_term::term::{svar::EthSvar, EthTerm};
use husky_place::PlaceRegistry;
use husky_regional_token::{RegionalTokenIdx, RegionalTokensData};
use husky_syn_decl::decl::{
    AssocItemSynNodeDecl, HasSynNodeDecl, ItemSynNodeDecl, TraitForTypeItemSynNodeDecl,
    TraitItemSynNodeDecl, TypeItemSynNodeDecl,
};
use husky_token_data::{IntegerLikeLiteralTokenData, LiteralTokenData, TokenData};
use husky_vfs::Toolchain;
use husky_vfs::VfsPathMenu;
use vec_like::VecPairMap;

pub(crate) struct SemaExprEngine<'a> {
    db: &'a ::salsa::Db,
    toolchain: Toolchain,
    item_path_menu: &'a ItemPathMenu,
    term_menu: &'a EthTermMenu,
    syn_expr_region: SynExprRegion,
    syn_expr_region_data: &'a SynExprRegionData,
    regional_tokens_data: RegionalTokensData<'a>,
    dec_term_region: &'a SynExprDecTermRegion,
    stack_location_registry: PlaceRegistry,
    sema_expr_arena: SemaExprArena,
    sema_stmt_arena: SemaStmtArena,
    pub(crate) sema_expr_roots: VecPairMap<SynExprIdx, (SemaExprIdx, SynExprRootKind)>,
    fly_term_region: FlyTermRegion,
    sema_expr_term_results: VecPairMap<SemaExprIdx, SemaExprTermResult<FlyTerm>>,
    symbol_terms: SymbolMap<FlyTerm>,
    symbol_tys: SymbolMap<SymbolType>,
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    pattern_expr_contracts: SynPatternExprMap<TermContract>,
    return_ty: Option<EthTerm>,
    pub(crate) unveiler: Unveiler,
    self_ty: Option<EthTerm>,
    self_value: Option<EthSvar>,
    self_value_ty: Option<FlyTerm>,
    self_lifetime: Option<EthSvar>,
    self_place: Option<EthSvar>,
    trai_in_use_items_table: TraitInUseItemsTable<'a>,
}

impl<'a> FlyTermEngine<'a> for SemaExprEngine<'a> {
    fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    fn fly_term_region(&self) -> &FlyTermRegion {
        &self.fly_term_region
    }

    fn expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    fn item_path_menu(&self) -> &'a ItemPathMenu {
        self.item_path_menu
    }

    fn term_menu(&self) -> &'a EthTermMenu {
        self.term_menu
    }

    fn trai_in_use_items_table(&self) -> TraitInUseItemsTable<'a> {
        self.trai_in_use_items_table
    }
}

impl<'a> FlyTermEngineMut<'a> for SemaExprEngine<'a> {
    fn stack_location_registry_mut(&mut self) -> &mut PlaceRegistry {
        &mut self.stack_location_registry
    }

    fn fly_term_region_mut(&mut self) -> &mut FlyTermRegion {
        &mut self.fly_term_region
    }
}

impl<'a> std::ops::Index<SynExprIdx> for SemaExprEngine<'a> {
    type Output = SynExprData;

    fn index(&self, index: SynExprIdx) -> &Self::Output {
        &self.syn_expr_region_data[index]
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn new(db: &'a ::salsa::Db, syn_expr_region: SynExprRegion) -> Self {
        let syn_expr_region_data = syn_expr_region.data(db);
        // todo: improve this
        let parent_expr_region = syn_expr_region_data.parent();
        let module_path = syn_expr_region_data.path().module_path(db);
        let return_ty = parent_expr_region
            .map(|parent_expr_region| {
                db.syn_expr_dec_term_region(parent_expr_region)
                    .expr_term(parent_expr_region.data(db).return_ty()?)
                    .ok()
            })
            .flatten()
            .map(|term| EthTerm::ty_from_dec(db, term).ok())
            .flatten();
        let dec_term_region = db.syn_expr_dec_term_region(syn_expr_region);
        let self_ty = dec_term_region
            .dec_symbol_region()
            .self_ty()
            .map(|self_ty| EthTerm::ty_from_dec(db, self_ty).ok())
            .flatten();
        let self_value = dec_term_region
            .dec_symbol_region()
            .self_value()
            .map(|self_value| EthSvar::from_dec(db, self_value).ok())
            .flatten();
        let mut stack_location_registry = Default::default();
        let self_lifetime = dec_term_region
            .dec_symbol_region()
            .self_lifetime()
            .map(|self_lifetime| EthSvar::from_dec(db, self_lifetime).ok())
            .flatten();
        let self_place = dec_term_region
            .dec_symbol_region()
            .self_place()
            .map(|self_place| EthSvar::from_dec(db, self_place).ok())
            .flatten();
        let self_value_ty = calc_self_value_ty(
            syn_expr_region_data,
            self_ty,
            self_place,
            db,
            &mut stack_location_registry,
        );
        let symbol_region = syn_expr_region_data.symbol_region();
        let pattern_expr_region = syn_expr_region_data.pattern_expr_region();
        let toolchain = syn_expr_region.toolchain(db);
        let parent_sema_expr_region =
            parent_expr_region.map(|parent_expr_region| db.sema_expr_region(parent_expr_region));
        let regional_tokens_data = match syn_expr_region_data.path() {
            SynNodeRegionPath::Snippet(_) => todo!(),
            SynNodeRegionPath::Decl(path) => path.decl_tokra_region(db).regional_tokens_data(db),
            SynNodeRegionPath::Defn(path) => path
                .defn_tokra_region(db)
                .expect("guaranteed")
                .tokens_data(db),
        };
        Self {
            db,
            toolchain,
            item_path_menu: item_path_menu(db, toolchain),
            term_menu: db.ethereal_term_menu(toolchain),
            syn_expr_region,
            syn_expr_region_data,
            dec_term_region,
            stack_location_registry,
            sema_expr_arena: SemaExprArena::default(),
            sema_stmt_arena: SemaStmtArena::default(),
            sema_expr_roots: Default::default(),
            fly_term_region: FlyTermRegion::new(
                parent_sema_expr_region.map(|r| r.data(db).fly_term_region()),
            ),
            sema_expr_term_results: Default::default(),
            symbol_terms: SymbolMap::new(
                parent_sema_expr_region
                    .map(|parent_sema_expr_region| parent_sema_expr_region.data(db).symbol_terms()),
                syn_expr_region_data.symbol_region(),
            ),
            symbol_tys: SymbolMap::new(
                parent_sema_expr_region
                    .map(|parent_sema_expr_region| parent_sema_expr_region.data(db).symbol_tys()),
                syn_expr_region_data.symbol_region(),
            ),
            pattern_expr_ty_infos: SynPatternExprMap::new(pattern_expr_region.pattern_expr_arena()),
            pattern_symbol_ty_infos: SynPatternSymbolMap::new(
                pattern_expr_region.pattern_symbol_arena(),
            ),
            return_ty,
            unveiler: Unveiler::Uninitialized,
            self_ty,
            self_value,
            self_value_ty,
            self_lifetime,
            self_place,
            pattern_expr_contracts: SynPatternExprMap::new(
                pattern_expr_region.pattern_expr_arena(),
            ),
            trai_in_use_items_table: TraitInUseItemsTable::query(db, module_path),
            regional_tokens_data,
        }
    }

    pub(crate) fn infer_all(&mut self) {
        self.infer_current_parameter_symbols();
        self.infer_all_exprs()
    }

    pub(crate) fn alloc_expr(
        &mut self,
        data_result: Result<SemaExprData, SemaExprDataError>,
        immediate_ty_result: Result<FlyTerm, SemaExprTypeError>,
        expectation_idx_and_ty: Option<(FlyTermExpectationIdx, FlyTerm)>,
    ) -> SemaExprIdx {
        let expr = self.sema_expr_arena.alloc_one(
            data_result,
            immediate_ty_result,
            expectation_idx_and_ty,
        );
        self.fly_term_region.resolve_as_much_as_possible(self.db());
        expr
    }

    pub(crate) fn alloc_stmt_batch(&mut self, batch: SemaStmtBatch) -> SemaStmtIdxRange {
        self.sema_stmt_arena.alloc_batch(batch)
    }

    pub(crate) fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    pub(crate) fn expr_region_data(&self) -> &SynExprRegionData {
        self.syn_expr_region_data
    }

    pub(crate) fn item_path_menu(&self) -> &ItemPathMenu {
        self.item_path_menu
    }

    pub(crate) fn toolchain(&self) -> Toolchain {
        self.toolchain
    }

    pub(crate) fn eth_term_menu(&self) -> &EthTermMenu {
        self.term_menu
    }

    pub(crate) fn token_data(&self, regional_token_idx: RegionalTokenIdx) -> TokenData {
        self.regional_tokens_data[regional_token_idx]
    }

    pub(crate) fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    pub(crate) fn add_symbol_ty(&mut self, symbol_idx: CurrentSynSymbolIdx, symbol_ty: SymbolType) {
        self.symbol_tys.insert_new(symbol_idx, symbol_ty)
    }

    pub(crate) fn sema_expr_arena(&self) -> &SemaExprArena {
        &self.sema_expr_arena
    }

    pub(crate) fn finish(mut self) -> SemaExprRegion {
        let db = self.db;
        self.fly_term_region
            .finalize_unresolved_term_table(db, self.term_menu);
        self.infer_extra_expr_terms_in_preparation_for_hir();
        SemaExprRegion::new(
            self.syn_expr_region_data
                .path()
                .region_path(db)
                .expect("should be some"),
            self.syn_expr_region,
            self.sema_expr_arena,
            self.sema_stmt_arena,
            self.sema_expr_roots,
            self.pattern_expr_ty_infos,
            self.pattern_symbol_ty_infos,
            self.sema_expr_term_results,
            self.symbol_tys,
            self.symbol_terms,
            self.fly_term_region,
            self.return_ty,
            self.self_ty,
            self.db,
        )
    }

    pub(crate) fn return_ty(&self) -> Option<EthTerm> {
        self.return_ty
    }

    pub(crate) fn symbol_tys(&self) -> &SymbolMap<SymbolType> {
        &self.symbol_tys
    }

    pub(crate) fn self_ty(&self) -> Option<EthTerm> {
        self.self_ty
    }

    pub(crate) fn self_value_ty(&self) -> Option<FlyTerm> {
        self.self_value_ty
    }
}

fn calc_self_value_ty(
    syn_expr_region_data: &SynExprRegionData,
    self_ty: Option<EthTerm>,
    self_place: Option<EthSvar>,
    db: &salsa::Db,
    registry: &mut PlaceRegistry,
) -> Option<FlyTerm> {
    fn method_fn_self_value_modifier_from_self_value_parameter(
        self_value_parameter: &Option<SelfValueParameterSyndicate>,
    ) -> SvarModifier {
        let Some(self_value_parameter) = self_value_parameter else {
            return SvarModifier::Pure;
        };
        SvarModifier::new(self_value_parameter.ephem_symbol_modifier_token_verse())
    }
    let self_ty: FlyTerm = self_ty?.into();
    let modifier = match syn_expr_region_data.path() {
        SynNodeRegionPath::Snippet(_) => None, // ad hoc
        SynNodeRegionPath::Decl(syn_node_path) | SynNodeRegionPath::Defn(syn_node_path) => {
            match syn_node_path.syn_node_decl(db) {
                ItemSynNodeDecl::AssocItem(syn_node_decl) => match syn_node_decl {
                    AssocItemSynNodeDecl::TypeItem(syn_node_decl) => match syn_node_decl {
                        TypeItemSynNodeDecl::MethodFn(syn_node_decl) => {
                            Some(method_fn_self_value_modifier_from_self_value_parameter(
                                syn_node_decl
                                    .parenate_parameters(db)
                                    .as_ref()
                                    .ok()?
                                    .self_value_parameter(),
                            ))
                        }
                        TypeItemSynNodeDecl::MemoizedField(_) => Some(SvarModifier::Le),
                        _ => None,
                    },
                    AssocItemSynNodeDecl::TraitItem(syn_node_decl) => match syn_node_decl {
                        TraitItemSynNodeDecl::MethodFn(_) => todo!(),
                        _ => None,
                    },
                    AssocItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => match syn_node_decl {
                        TraitForTypeItemSynNodeDecl::MethodFn(syn_node_decl) => {
                            Some(method_fn_self_value_modifier_from_self_value_parameter(
                                syn_node_decl
                                    .parenate_parameter_decl_list(db)
                                    .as_ref()
                                    .ok()?
                                    .self_value_parameter(),
                            ))
                        }
                        _ => None,
                    },
                    AssocItemSynNodeDecl::IllFormedItem(_) => None,
                },
                _ => None,
            }
        }
    }?;
    let place = match modifier {
        SvarModifier::Pure => FlyPlace::StackPure {
            location: registry.issue_new(),
        },
        SvarModifier::Owned => FlyPlace::ImmutableStackOwned {
            location: registry.issue_new(),
        },
        SvarModifier::Mut => FlyPlace::MutableStackOwned {
            location: registry.issue_new(),
        },
        SvarModifier::Ref => todo!(),
        SvarModifier::RefMut => todo!(),
        SvarModifier::Const => todo!(),
        SvarModifier::Ambersand(_) => FlyPlace::Ref {
            guard: Left(registry.issue_new()),
        },
        SvarModifier::AmbersandMut(_) => FlyPlace::RefMut {
            guard: Left(registry.issue_new()),
        },
        SvarModifier::Le => FlyPlace::Leashed,
        SvarModifier::Tilde => FlyPlace::Leashed,
        SvarModifier::At => FlyPlace::EtherealSymbol(self_place?),
    };
    Some(self_ty.with_place(place))
}
