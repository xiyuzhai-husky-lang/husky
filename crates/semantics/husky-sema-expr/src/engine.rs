mod expect;
mod expr_term;
mod expr_ty;
mod stmt;
mod symbol;
#[macro_use]
mod utils;
mod branch_ty_merger;
mod pattern_ty;

pub(crate) use self::branch_ty_merger::*;
pub use self::expr_ty::*;
pub(crate) use self::utils::*;

use self::symbol::*;
use crate::*;
use husky_entity_syn_tree::helpers::{
    tokra_region::{HasDeclTokraRegion, HasSynDefnTokraRegion},
    TraitInUseItemsTable,
};
use husky_ethereal_signature::HasEtherealSignatureTemplate;
use husky_opr::PrefixOpr;
use husky_print_utils::p;
use husky_regional_token::{RegionalTokenIdx, RegionalTokensData};
use husky_token_data::{IntegerLikeLiteralData, LiteralData, TokenData};
use husky_vfs::Toolchain;
use husky_vfs::VfsPathMenu;
use vec_like::VecPairMap;

pub(crate) struct SemaExprEngine<'a> {
    db: &'a dyn SemaExprDb,
    toolchain: Toolchain,
    item_path_menu: &'a ItemPathMenu,
    term_menu: &'a EtherealTermMenu,
    syn_expr_region_data: &'a SynExprRegionData,
    regional_tokens_data: RegionalTokensData<'a>,
    declarative_term_region: &'a DeclarativeTermRegion,
    sema_expr_arena: SemaExprArena,
    sema_stmt_arena: SemaStmtArena,
    syn_expr_root_sema_expr_idx_table: VecPairMap<SynExprIdx, SemaExprIdx>,
    fluffy_term_region: FluffyTermRegion,
    sema_expr_term_results: VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>>,
    symbol_terms: SymbolMap<FluffyTerm>,
    symbol_tys: SymbolMap<SymbolType>,
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    pattern_expr_contracts: SynPatternExprMap<Contract>,
    return_ty: Option<EtherealTerm>,
    unveiler: Unveiler,
    self_ty_term: Option<EtherealTerm>,
    trai_in_use_items_table: EntityTreeResultRef<'a, TraitInUseItemsTable<'a>>,
}

impl<'a> FluffyTermEngine<'a> for SemaExprEngine<'a> {
    fn db(&self) -> &'a dyn FluffyTermDb {
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

    fn trai_in_use_items_table(&self) -> EntityTreeResultRef<'a, TraitInUseItemsTable<'a>> {
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
    pub(crate) fn new(db: &'a dyn SemaExprDb, syn_expr_region: SynExprRegion) -> Self {
        let expr_region_data = syn_expr_region.data(db);
        // todo: improve this
        let parent_expr_region = expr_region_data.parent();
        let module_path = expr_region_data.path().module_path(db);
        let return_ty = parent_expr_region
            .map(|parent_expr_region| {
                db.declarative_term_region(parent_expr_region)
                    .expr_term(parent_expr_region.data(db).return_ty()?)
                    .ok()
            })
            .flatten()
            .map(|term| EtherealTerm::ty_from_declarative(db, term).ok())
            .flatten();
        let self_value_ty = match expr_region_data.path() {
            RegionPath::Snippet(_) => None,
            RegionPath::Decl(node_path) | RegionPath::Defn(node_path) => {
                let Some(item_path) = node_path.path(db) else {
                    unreachable!("node_path = {:?}", node_path.debug(db))
                };
                item_path
                    .ethereal_signature_template(db)
                    .ok()
                    .map(|st| st.self_ty(db))
                    .flatten()
            }
        };
        // parent_expr_region
        //     .map(|parent_expr_region| {
        //         db.declarative_term_region(parent_expr_region)
        //             .expr_term(parent_expr_region.data(db).self_ty()?)
        //             .ok()
        //     })
        //     .flatten()
        //     .map(|term| EtherealTerm::ty_from_declarative(db, term).ok())
        //     .flatten();
        let symbol_region = expr_region_data.symbol_region();
        let pattern_expr_region = expr_region_data.pattern_expr_region();
        let toolchain = syn_expr_region.toolchain(db);
        let parent_expr_ty_region =
            parent_expr_region.map(|parent_expr_region| db.sema_expr_region(parent_expr_region));
        let regional_tokens_data = match expr_region_data.path() {
            RegionPath::Snippet(_) => todo!(),
            RegionPath::Decl(path) => path.decl_tokra_region(db).tokens_data(db),
            RegionPath::Defn(path) => path
                .defn_tokra_region(db)
                .expect("guaranteed")
                .tokens_data(db),
        };
        Self {
            db,
            toolchain,
            item_path_menu: db.item_path_menu(toolchain),
            term_menu: db.ethereal_term_menu(toolchain),
            syn_expr_region_data: expr_region_data,
            declarative_term_region: db.declarative_term_region(syn_expr_region),
            sema_expr_arena: SemaExprArena::default(),
            sema_stmt_arena: SemaStmtArena::default(),
            syn_expr_root_sema_expr_idx_table: Default::default(),
            fluffy_term_region: FluffyTermRegion::new(
                parent_expr_ty_region.map(|r| r.fluffy_term_region()),
            ),
            sema_expr_term_results: Default::default(),
            symbol_terms: SymbolMap::new(
                parent_expr_ty_region
                    .map(|parent_expr_ty_region| parent_expr_ty_region.symbol_terms()),
                expr_region_data.symbol_region(),
            ),
            symbol_tys: SymbolMap::new(
                parent_expr_ty_region
                    .map(|parent_expr_ty_region| parent_expr_ty_region.symbol_tys()),
                expr_region_data.symbol_region(),
            ),
            pattern_expr_ty_infos: SynPatternExprMap::new(pattern_expr_region.pattern_expr_arena()),
            pattern_symbol_ty_infos: SynPatternSymbolMap::new(
                pattern_expr_region.pattern_symbol_arena(),
            ),
            return_ty,
            unveiler: Unveiler::Uninitialized,
            self_ty_term: self_value_ty,
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
        for root in self.syn_expr_region_data.roots() {
            let sema_expr_idx = match root.kind() {
                ExprRootKind::SelfType
                | ExprRootKind::ReturnType
                | ExprRootKind::ReturnType
                | ExprRootKind::PropsStructFieldType { .. }
                | ExprRootKind::TupleStructFieldType
                | ExprRootKind::ConstantImplicitParameterType
                | ExprRootKind::ExplicitParameterType
                | ExprRootKind::AssociatedTypeTerm => self.build_sema_expr(
                    root.syn_expr_idx(),
                    ExpectEqsCategory::new_expect_eqs_ty_kind(),
                ),
                ExprRootKind::Trait => self.build_sema_expr(root.syn_expr_idx(), ExpectAnyOriginal),
                ExprRootKind::BlockExpr => match self.return_ty {
                    Some(return_ty) => self.build_sema_expr(
                        root.syn_expr_idx(),
                        ExpectCoersion::new_move(return_ty.into()),
                    ),
                    None => self.build_sema_expr(root.syn_expr_idx(), ExpectAnyDerived),
                },
                ExprRootKind::FieldBindInitialValue { ty_syn_expr_idx }
                | ExprRootKind::ExplicitParameterDefaultValue { ty_syn_expr_idx } => {
                    let ty_sema_expr_idx =
                        self.syn_expr_root_sema_expr_idx_table[ty_syn_expr_idx].1;
                    match self.infer_expr_term(ty_sema_expr_idx) {
                        Some(ty) => {
                            self.build_sema_expr(root.syn_expr_idx(), ExpectCoersion::new_move(ty))
                        }
                        _ => todo!(),
                    }
                }
                ExprRootKind::ReturnExpr
                | ExprRootKind::Condition
                | ExprRootKind::HtmlArgumentExpr
                | ExprRootKind::LetStmtType
                | ExprRootKind::LetStmtInitialValue
                | ExprRootKind::EvalExpr => continue,
                ExprRootKind::Snippet => todo!(),
                ExprRootKind::Traits =>
                /* ad hoc */
                {
                    continue
                }
                // todo!(),
                ExprRootKind::ValExpr => todo!(),
            };
            self.syn_expr_root_sema_expr_idx_table
                .insert_new((root.syn_expr_idx(), sema_expr_idx))
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
            self.db,
            self.syn_expr_region_data.path(),
            self.sema_expr_arena,
            self.sema_stmt_arena,
            self.syn_expr_root_sema_expr_idx_table,
            self.pattern_expr_ty_infos,
            self.pattern_symbol_ty_infos,
            self.sema_expr_term_results,
            self.symbol_terms,
            self.symbol_tys,
            self.fluffy_term_region,
            self.return_ty,
            self.self_ty_term,
        )
    }

    pub(crate) fn db(&self) -> &'a dyn SemaExprDb {
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

    pub(crate) fn term_menu(&self) -> &EtherealTermMenu {
        self.term_menu
    }

    fn token_data(&self, regional_token_idx: RegionalTokenIdx) -> TokenData {
        self.regional_tokens_data[regional_token_idx]
    }

    pub(crate) fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    pub(crate) fn add_symbol_ty(&mut self, symbol_idx: SynCurrentSymbolIdx, symbol_ty: SymbolType) {
        self.symbol_tys.insert_new(symbol_idx, symbol_ty)
    }
}
