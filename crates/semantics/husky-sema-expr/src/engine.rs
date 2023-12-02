mod expect;
mod expr_term;
mod sema_expr;
mod stmt;
mod symbol;
#[macro_use]
mod utils;
mod branch_ty_merger;
mod pattern_ty;

pub(crate) use self::branch_ty_merger::*;
pub use self::sema_expr::*;
pub(crate) use self::utils::*;

use self::symbol::*;
use crate::*;
use husky_entity_syn_tree::helpers::TraitInUseItemsTable;
use husky_ethereal_signature::HasEtherealSignatureTemplate;
use husky_print_utils::p;
use husky_regional_token::{RegionalTokenIdx, RegionalTokensData};
use husky_token_data::{IntegerLikeLiteralData, LiteralData, TokenData};
use husky_vfs::Toolchain;
use husky_vfs::VfsPathMenu;
use vec_like::VecPairMap;

pub(crate) struct SemaExprEngine<'a> {
    db: &'a ::salsa::Db,
    toolchain: Toolchain,
    item_path_menu: &'a ItemPathMenu,
    term_menu: &'a EtherealTermMenu,
    syn_expr_region: SynExprRegion,
    syn_expr_region_data: &'a SynExprRegionData,
    regional_tokens_data: RegionalTokensData<'a>,
    declarative_term_region: &'a DeclarativeTermRegion,
    sema_expr_arena: SemaExprArena,
    sema_stmt_arena: SemaStmtArena,
    sema_expr_roots: VecPairMap<SynExprIdx, (SemaExprIdx, SynExprRootKind)>,
    fluffy_term_region: FluffyTermRegion,
    sema_expr_term_results: VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>>,
    symbol_terms: SymbolMap<FluffyTerm>,
    symbol_tys: SymbolMap<SymbolType>,
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    pattern_expr_contracts: SynPatternExprMap<TermContract>,
    return_ty: Option<EtherealTerm>,
    unveiler: Unveiler,
    self_ty: Option<EtherealTerm>,
    self_value: Option<EtherealTermSymbol>,
    self_lifetime: Option<EtherealTermSymbol>,
    self_place: Option<EtherealTermSymbol>,
    trai_in_use_items_table: TraitInUseItemsTable<'a>,
}

impl<'a> FluffyTermEngine<'a> for SemaExprEngine<'a> {
    fn db(&self) -> &'a ::salsa::Db {
        self.db
    }
    fn fluffy_term_region(&self) -> &FluffyTermRegion {
        &self.fluffy_term_region
    }

    fn fluffy_term_region_mut(&mut self) -> &mut FluffyTermRegion {
        &mut self.fluffy_term_region
    }

    fn expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    fn item_path_menu(&self) -> &'a ItemPathMenu {
        self.item_path_menu
    }

    fn term_menu(&self) -> &'a EtherealTermMenu {
        self.term_menu
    }

    fn trai_in_use_items_table(&self) -> TraitInUseItemsTable<'a> {
        self.trai_in_use_items_table
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
                db.declarative_term_region(parent_expr_region)
                    .expr_term(parent_expr_region.data(db).return_ty()?)
                    .ok()
            })
            .flatten()
            .map(|term| EtherealTerm::ty_from_declarative(db, term).ok())
            .flatten();
        let declarative_term_region = db.declarative_term_region(syn_expr_region);
        let self_ty = declarative_term_region
            .term_symbol_region()
            .self_ty()
            .map(|self_ty| EtherealTerm::ty_from_declarative(db, self_ty).ok())
            .flatten();
        let self_value = declarative_term_region
            .term_symbol_region()
            .self_value()
            .map(|self_value| EtherealTermSymbol::from_declarative(db, self_value).ok())
            .flatten();
        let self_lifetime = declarative_term_region
            .term_symbol_region()
            .self_lifetime()
            .map(|self_lifetime| EtherealTermSymbol::from_declarative(db, self_lifetime).ok())
            .flatten();
        let self_place = declarative_term_region
            .term_symbol_region()
            .self_place()
            .map(|self_place| EtherealTermSymbol::from_declarative(db, self_place).ok())
            .flatten();
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
            declarative_term_region,
            sema_expr_arena: SemaExprArena::default(),
            sema_stmt_arena: SemaStmtArena::default(),
            sema_expr_roots: Default::default(),
            fluffy_term_region: FluffyTermRegion::new(
                parent_sema_expr_region.map(|r| r.data(db).fluffy_term_region()),
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

    fn infer_all_exprs(&mut self) {
        for root in self.syn_expr_region_data.syn_expr_roots() {
            let sema_expr_idx = match root.kind() {
                SynExprRootKind::SelfType
                | SynExprRootKind::ReturnType
                | SynExprRootKind::ReturnType
                | SynExprRootKind::PropsStructFieldType { .. }
                | SynExprRootKind::TupleStructFieldType
                | SynExprRootKind::ConstantImplicitParameterType
                | SynExprRootKind::ExplicitParameterType
                | SynExprRootKind::AssociatedTypeTerm => {
                    let sema_expr_idx = self.build_sema_expr(
                        root.syn_expr_idx(),
                        ExpectEqsCategory::new_expect_eqs_ty_kind(),
                    );
                    self.infer_expr_term(sema_expr_idx);
                    sema_expr_idx
                }
                SynExprRootKind::Trait => {
                    let sema_expr_idx =
                        self.build_sema_expr(root.syn_expr_idx(), ExpectAnyOriginal);
                    self.infer_expr_term(sema_expr_idx);
                    sema_expr_idx
                }
                SynExprRootKind::BlockExpr => match self.return_ty {
                    Some(return_ty) => self.build_sema_expr(
                        root.syn_expr_idx(),
                        ExpectCoersion::new_move(return_ty.into()),
                    ),
                    None => self.build_sema_expr(root.syn_expr_idx(), ExpectAnyDerived),
                },
                SynExprRootKind::FieldBindInitialValue { ty_syn_expr_idx }
                | SynExprRootKind::ExplicitParameterDefaultValue { ty_syn_expr_idx } => {
                    let (ty_sema_expr_idx, _) = self.sema_expr_roots[ty_syn_expr_idx].1;
                    match self.infer_expr_term(ty_sema_expr_idx) {
                        Some(ty) => {
                            self.build_sema_expr(root.syn_expr_idx(), ExpectCoersion::new_move(ty))
                        }
                        _ => todo!(),
                    }
                }
                SynExprRootKind::ReturnExpr
                | SynExprRootKind::Condition
                | SynExprRootKind::HtmlArgumentExpr
                | SynExprRootKind::LetStmtType
                | SynExprRootKind::LetStmtInitialValue
                | SynExprRootKind::EvalExpr => continue,
                SynExprRootKind::Snippet => todo!(),
                SynExprRootKind::ValExpr => todo!(),
            };
            self.sema_expr_roots
                .insert_new((root.syn_expr_idx(), (sema_expr_idx, root.kind())))
                .expect("impossible")
        }
    }

    pub(crate) fn finish(mut self) -> SemaExprRegion {
        // ad hoc, todo: enforce this
        // for expr_idx in self.expr_region_data.expr_arena().index_iter() {
        //     print_debug_expr!(self, expr_idx);
        //     assert!(self.expr_ty_infos.has(expr_idx))
        // }
        self.fluffy_term_region
            .finalize_unresolved_term_table(self.db, self.term_menu);
        self.infer_extra_expr_terms_in_preparation_for_hir();
        SemaExprRegion::new(
            self.syn_expr_region_data.path(),
            self.syn_expr_region,
            self.sema_expr_arena,
            self.sema_stmt_arena,
            self.sema_expr_roots,
            self.pattern_expr_ty_infos,
            self.pattern_symbol_ty_infos,
            self.sema_expr_term_results,
            self.symbol_tys,
            self.symbol_terms,
            self.fluffy_term_region,
            self.return_ty,
            self.self_ty,
            self.db,
        )
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

    pub(crate) fn eth_term_menu(&self) -> &EtherealTermMenu {
        self.term_menu
    }

    fn token_data(&self, regional_token_idx: RegionalTokenIdx) -> TokenData {
        self.regional_tokens_data[regional_token_idx]
    }

    pub(crate) fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    pub(crate) fn add_symbol_ty(&mut self, symbol_idx: CurrentSynSymbolIdx, symbol_ty: SymbolType) {
        self.symbol_tys.insert_new(symbol_idx, symbol_ty)
    }
}
