mod expect;
mod expr_term;
mod expr_ty;
mod stmt;
mod symbol;
mod utils;

use husky_opn_syntax::PrefixOpr;
use husky_print_utils::p;
use husky_token::{IntegerLikeLiteral, Literal, Token, TokenIdx, TokenSheetData};
use husky_vfs::Toolchain;
use symbol::*;

use crate::*;

pub(crate) struct ExprTypeEngine<'a> {
    db: &'a dyn ExprTypeDb,
    toolchain: Toolchain,
    entity_path_menu: &'a EntityPathMenu,
    term_menu: &'a TermMenu,
    reduced_term_menu: ReducedTermMenu<'a>,
    token_sheet_data: &'a TokenSheetData,
    expr_region_data: &'a ExprRegionData,
    signature_term_region: &'a SignatureTermRegion,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
    expr_terms: ExprMap<ExprTermResult<LocalTerm>>,
    inherited_symbol_tys: InheritedSymbolMap<ReducedTerm>,
    current_symbol_tys: CurrentSymbolMap<LocalTerm>,
    local_term_table: LocalTermTable,
    pattern_expr_ty_infos: PatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
    return_ty: Option<ReducedTerm>,
    self_ty: Option<ReducedTerm>,
}

impl<'a> std::ops::Index<ExprIdx> for ExprTypeEngine<'a> {
    type Output = Expr;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_region_data[index]
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn new(db: &'a dyn ExprTypeDb, expr_region: ExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        // todo: improve this
        let return_ty = expr_region_data
            .parent()
            .map(|parent| {
                db.signature_term_region(parent)
                    .expr_term(parent.data(db).return_ty()?)
                    .ok()
            })
            .flatten()
            .map(|term| db.reduced_term(term));
        // todo: improve this
        let self_ty = expr_region_data
            .parent()
            .map(|parent| {
                db.signature_term_region(parent)
                    .expr_term(parent.data(db).self_ty()?)
                    .ok()
            })
            .flatten()
            .map(|term| db.reduced_term(term));
        let symbol_region = expr_region_data.symbol_region();
        let pattern_expr_region = expr_region_data.pattern_expr_region();
        let toolchain = expr_region.toolchain(db);
        let reduced_term_menu = db.reduced_term_menu(toolchain).unwrap();
        Self {
            db,
            toolchain,
            entity_path_menu: db.entity_path_menu(toolchain).unwrap(),
            term_menu: reduced_term_menu.term_menu(),
            reduced_term_menu,
            token_sheet_data: db
                .token_sheet_data(expr_region_data.path().module_path(db))
                .unwrap(),
            expr_region_data,
            signature_term_region: db.signature_term_region(expr_region),
            expr_ty_infos: ExprMap::new(expr_region_data.expr_arena()),
            expr_terms: ExprMap::new(expr_region_data.expr_arena()),
            inherited_symbol_tys: InheritedSymbolMap::new(symbol_region.inherited_symbol_arena()),
            current_symbol_tys: CurrentSymbolMap::new(symbol_region.current_symbol_arena()),
            local_term_table: Default::default(),
            return_ty,
            pattern_expr_ty_infos: PatternExprMap::new(pattern_expr_region.pattern_expr_arena()),
            pattern_symbol_ty_infos: PatternSymbolMap::new(
                pattern_expr_region.pattern_symbol_arena(),
            ),
            self_ty,
        }
    }

    pub(crate) fn infer_all(&mut self) {
        self.infer_all_parameter_symbols();
        self.infer_all_exprs();
    }

    fn infer_all_exprs(&mut self) {
        for root in self.expr_region_data.roots() {
            let ty = self.infer_new_expr_ty(root.expr(), ExpectInsSort::default());
            // todo: check coherence
        }
    }

    pub(crate) fn finish(mut self) -> ExprTypeRegion {
        self.finalize_unresolved_term_table();
        ExprTypeRegion::new(
            self.db,
            self.reduced_term_menu,
            self.expr_region_data.path(),
            self.expr_ty_infos,
            self.expr_terms,
            self.inherited_symbol_tys,
            self.current_symbol_tys,
            self.local_term_table,
            self.return_ty,
            self.self_ty,
        )
    }

    pub(crate) fn db(&self) -> &'a dyn ExprTypeDb {
        self.db
    }

    pub(crate) fn reduced_term_menu(&self) -> ReducedTermMenu<'a> {
        self.reduced_term_menu
    }

    pub(crate) fn local_term_table(&self) -> &LocalTermTable {
        &self.local_term_table
    }

    pub(crate) fn local_term_table_mut(&mut self) -> &mut LocalTermTable {
        &mut self.local_term_table
    }

    pub(crate) fn expr_region_data(&self) -> &ExprRegionData {
        self.expr_region_data
    }

    pub(crate) fn entity_path_menu(&self) -> &EntityPathMenu {
        self.entity_path_menu
    }

    pub(crate) fn toolchain(&self) -> Toolchain {
        self.toolchain
    }
}
