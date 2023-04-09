mod expect;
mod expr_term;
mod expr_ty;
mod stmt;
mod symbol;
#[macro_use]
mod utils;
mod pattern_ty;
mod symbol_liason;

pub(crate) use self::utils::*;

use self::symbol::*;
use husky_opn_syntax::PrefixOpr;
use husky_print_utils::p;
use husky_token::{IntegerLikeLiteral, Literal, Token, TokenIdx, TokenSheetData};
use husky_ty_expectation::TermTypeExpectation;
use husky_vfs::Toolchain;

use crate::*;

pub(crate) struct ExprTypeEngine<'a> {
    db: &'a dyn ExprTypeDb,
    toolchain: Toolchain,
    entity_path_menu: &'a EntityPathMenu,
    term_menu: &'a TermMenu,
    token_sheet_data: &'a TokenSheetData,
    expr_region_data: &'a ExprRegionData,
    signature_term_region: &'a SignatureRegion,
    fluffy_term_region: FluffyTermRegion,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
    extra_expr_errors: Vec<(ExprIdx, ExprTypeError)>,
    expr_terms: ExprMap<ExprTermResult<FluffyTerm>>,
    inherited_symbol_terms: InheritedSymbolMap<TermSymbol>,
    inherited_symbol_place_tys: InheritedSymbolMap<PlaceType>,
    current_symbol_terms: CurrentSymbolMap<TermSymbol>,
    current_symbol_place_tys: CurrentSymbolMap<PlaceType>,
    pattern_expr_ty_infos: PatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
    return_ty: Option<Term>,
    self_ty: Option<Term>,
}

impl<'a> FluffyTermEngine<'a> for ExprTypeEngine<'a> {
    fn db(&self) -> &'a dyn FluffyTermDb {
        self.db
    }
    fn fluffy_term_region(&self) -> &FluffyTermRegion {
        &self.fluffy_term_region
    }

    fn fluffy_term_region_mut(&mut self) -> &mut FluffyTermRegion {
        &mut self.fluffy_term_region
    }

    fn expr_region_data(&self) -> &'a ExprRegionData {
        self.expr_region_data
    }
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
            .map(|term| Term::ty_from_raw(db, term).ok())
            .flatten();
        // todo: improve this
        let self_ty = expr_region_data
            .parent()
            .map(|parent| {
                db.signature_term_region(parent)
                    .expr_term(parent.data(db).self_ty()?)
                    .ok()
            })
            .flatten()
            .map(|term| Term::ty_from_raw(db, term).ok())
            .flatten();
        let symbol_region = expr_region_data.symbol_region();
        let pattern_expr_region = expr_region_data.pattern_expr_region();
        let toolchain = expr_region.toolchain(db);
        Self {
            db,
            toolchain,
            entity_path_menu: db.entity_path_menu(toolchain),
            term_menu: db.term_menu(toolchain),
            token_sheet_data: db
                .token_sheet_data(expr_region_data.path().module_path(db))
                .unwrap(),
            expr_region_data,
            signature_term_region: db.signature_term_region(expr_region),
            fluffy_term_region: Default::default(),
            expr_ty_infos: ExprMap::new(expr_region_data.expr_arena()),
            extra_expr_errors: vec![],
            expr_terms: ExprMap::new(expr_region_data.expr_arena()),
            inherited_symbol_terms: InheritedSymbolMap::new(symbol_region.inherited_symbol_arena()),
            inherited_symbol_place_tys: InheritedSymbolMap::new(
                symbol_region.inherited_symbol_arena(),
            ),
            current_symbol_terms: CurrentSymbolMap::new(symbol_region.current_symbol_arena()),
            current_symbol_place_tys: CurrentSymbolMap::new(symbol_region.current_symbol_arena()),
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
            match root.kind() {
                ExprRootKind::SelfType
                | ExprRootKind::ReturnType
                | ExprRootKind::VarType
                | ExprRootKind::FieldType => self.infer_new_expr_ty_discarded(
                    root.expr(),
                    ExpectEqsCategory::new_expect_eqs_ty_kind(),
                ),
                ExprRootKind::Trait => {
                    self.infer_new_expr_ty_discarded(root.expr(), ExpectAnyOriginal)
                }
                ExprRootKind::BlockExpr => match self.return_ty {
                    Some(return_ty) => self.infer_new_expr_ty_discarded(
                        root.expr(),
                        ExpectImplicitlyConvertible::new_transient(return_ty.into()),
                    ),
                    None => self.infer_new_expr_ty_discarded(root.expr(), ExpectAnyDerived),
                },
            };
        }
    }

    fn add_expr_ty_error(&mut self, expr_idx: ExprIdx, expr_ty_error: impl Into<ExprTypeError>) {
        self.extra_expr_errors
            .push((expr_idx, expr_ty_error.into()))
    }

    pub(crate) fn finish(mut self) -> ExprTypeRegion {
        // ad hoc, todo: enforce this
        // for expr_idx in self.expr_region_data.expr_arena().index_iter() {
        //     print_debug_expr!(self, expr_idx);
        //     assert!(self.expr_ty_infos.has(expr_idx))
        // }
        self.fluffy_term_region
            .finalize_unresolved_term_table(self.db);
        ExprTypeRegion::new(
            self.db,
            self.expr_region_data.path(),
            self.expr_ty_infos,
            self.extra_expr_errors,
            self.expr_terms,
            self.inherited_symbol_terms,
            self.inherited_symbol_place_tys,
            self.current_symbol_terms,
            self.current_symbol_place_tys,
            self.fluffy_term_region,
            self.return_ty,
            self.self_ty,
        )
    }

    pub(crate) fn db(&self) -> &'a dyn ExprTypeDb {
        self.db
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

    pub(crate) fn term_menu(&self) -> &TermMenu {
        self.term_menu
    }
}
