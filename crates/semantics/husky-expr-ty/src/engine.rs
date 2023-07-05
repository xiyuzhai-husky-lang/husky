mod expect;
mod expr_term;
mod expr_ty;
mod stmt;
mod symbol;
#[macro_use]
mod utils;
mod pattern_ty;

pub(crate) use self::utils::*;

use self::symbol::*;
use crate::*;
use husky_opn_syntax::PrefixOpr;
use husky_print_utils::p;
use husky_token::{IntegerLikeLiteral, Literal, Token, TokenIdx, TokenSheetData};
use husky_ty_expectation::TermTypeExpectation;
use husky_vfs::Toolchain;

pub(crate) struct ExprTypeEngine<'a> {
    db: &'a dyn ExprTypeDb,
    toolchain: Toolchain,
    entity_path_menu: &'a EntityPathMenu,
    term_menu: &'a EtherealTermMenu,
    token_sheet_data: &'a TokenSheetData,
    expr_region_data: &'a ExprRegionData,
    declarative_term_region: &'a DeclarativeTermRegion,
    fluffy_term_region: FluffyTermRegion,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
    extra_expr_errors: Vec<(ExprIdx, ExprTypeError)>,
    expr_terms: ExprMap<ExprTermResult<FluffyTerm>>,
    symbol_terms: SymbolMap<FluffyTerm>,
    symbol_tys: SymbolMap<SymbolType>,
    pattern_expr_ty_infos: PatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
    pattern_expr_contracts: PatternExprMap<Contract>,
    return_ty: Option<EtherealTerm>,
    self_ty: Option<EtherealTerm>,
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

    fn term_menu(&self) -> &'a EtherealTermMenu {
        self.term_menu
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
        let parent_expr_region = expr_region_data.parent();
        let return_ty = parent_expr_region
            .map(|parent_expr_region| {
                db.declarative_term_region(parent_expr_region)
                    .expr_term(parent_expr_region.data(db).return_ty()?)
                    .ok()
            })
            .flatten()
            .map(|term| EtherealTerm::ty_from_declarative(db, term).ok())
            .flatten();
        // todo: improve this
        let self_ty = parent_expr_region
            .map(|parent_expr_region| {
                db.declarative_term_region(parent_expr_region)
                    .expr_term(parent_expr_region.data(db).self_ty()?)
                    .ok()
            })
            .flatten()
            .map(|term| EtherealTerm::ty_from_declarative(db, term).ok())
            .flatten();
        let symbol_region = expr_region_data.symbol_region();
        let pattern_expr_region = expr_region_data.pattern_expr_region();
        let toolchain = expr_region.toolchain(db);
        let parent_expr_ty_region =
            parent_expr_region.map(|parent_expr_region| db.expr_ty_region(parent_expr_region));
        Self {
            db,
            toolchain,
            entity_path_menu: db.entity_path_menu(toolchain),
            term_menu: db.ethereal_term_menu(toolchain),
            token_sheet_data: expr_region_data.region_path().token_sheet_data(db).unwrap(),
            expr_region_data,
            declarative_term_region: db.declarative_term_region(expr_region),
            fluffy_term_region: FluffyTermRegion::new(
                parent_expr_ty_region.map(|r| r.fluffy_term_region()),
            ),
            expr_ty_infos: ExprMap::new(expr_region_data.expr_arena()),
            extra_expr_errors: vec![],
            expr_terms: ExprMap::new(expr_region_data.expr_arena()),
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
            return_ty,
            pattern_expr_ty_infos: PatternExprMap::new(pattern_expr_region.pattern_expr_arena()),
            pattern_symbol_ty_infos: PatternSymbolMap::new(
                pattern_expr_region.pattern_symbol_arena(),
            ),
            self_ty,
            pattern_expr_contracts: PatternExprMap::new(pattern_expr_region.pattern_expr_arena()),
        }
    }

    pub(crate) fn infer_all(&mut self) {
        self.infer_current_parameter_symbols();
        self.infer_all_exprs();
    }

    fn infer_all_exprs(&mut self) {
        for root in self.expr_region_data.roots() {
            match root.kind() {
                ExprRootKind::SelfType
                | ExprRootKind::ReturnType
                | ExprRootKind::VarType
                | ExprRootKind::PropsStructFieldType { .. }
                | ExprRootKind::TupleStructFieldType
                | ExprRootKind::ConstantImplicitParameterType
                | ExprRootKind::ExplicitParameterType => self.infer_new_expr_ty_discarded(
                    root.expr_idx(),
                    ExpectEqsCategory::new_expect_eqs_ty_kind(),
                ),
                ExprRootKind::Trait => {
                    self.infer_new_expr_ty_discarded(root.expr_idx(), ExpectAnyOriginal)
                }
                ExprRootKind::BlockExpr => match self.return_ty {
                    Some(return_ty) => self.infer_new_expr_ty_discarded(
                        root.expr_idx(),
                        ExpectImplicitlyConvertible::new_move(return_ty.into()),
                    ),
                    None => self.infer_new_expr_ty_discarded(root.expr_idx(), ExpectAnyDerived),
                },
                ExprRootKind::FieldBindInitialValue { ty_expr_idx }
                | ExprRootKind::ExplicitParameterDefaultValue { ty_expr_idx } => {
                    match self.infer_new_expr_term(ty_expr_idx) {
                        Some(ty) => self.infer_new_expr_ty_discarded(
                            root.expr_idx(),
                            ExpectImplicitlyConvertible::new_move(ty),
                        ),
                        _ => todo!(),
                    }
                }
                ExprRootKind::ReturnExpr
                | ExprRootKind::Condition
                | ExprRootKind::HtmlArgumentExpr
                | ExprRootKind::LetStmtType
                | ExprRootKind::LetStmtInitialValue
                | ExprRootKind::EvalExpr => (),
                ExprRootKind::Snippet => todo!(),
                ExprRootKind::Traits =>
                /* ad hoc */
                {
                    ()
                }
                // todo!(),
                ExprRootKind::ValExpr => todo!(),
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
            self.expr_region_data.region_path(),
            self.expr_ty_infos,
            self.extra_expr_errors,
            self.expr_terms,
            self.symbol_terms,
            self.symbol_tys,
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

    pub(crate) fn term_menu(&self) -> &EtherealTermMenu {
        self.term_menu
    }
}
