use crate::*;
use husky_print_utils::p;
use husky_token::{HasTokenIdxRange, RangedTokenSheet, TokenIdxRange, TokenSheetData};
use husky_vfs::ModulePath;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb, jar = ExprJar)]
pub struct ExprRangeRegion {
    entity_path_expr_ranges: Vec<TokenIdxRange>,
    pattern_expr_ranges: Vec<TokenIdxRange>,
    expr_ranges: Vec<TokenIdxRange>,
    stmt_ranges: StmtMap<TokenIdxRange>,
}

#[salsa::tracked(jar = ExprJar, return_ref)]
pub(crate) fn expr_range_region(db: &dyn ExprDb, expr_region: ExprRegion) -> ExprRangeRegion {
    ExprRangeCalculator::new(db, expr_region).calc_all()
}

// #[test]
// fn expr_range_sheet_works() {
//     use tests::*;
//     DB::default().ast_expect_test_debug_with_db("expr_range_sheet", todo!());
// }

impl std::ops::Index<EntityPathExprIdx> for ExprRangeRegion {
    type Output = TokenIdxRange;

    fn index(&self, index: EntityPathExprIdx) -> &Self::Output {
        &self.entity_path_expr_ranges[index.raw()]
    }
}

impl std::ops::Index<PatternExprIdx> for ExprRangeRegion {
    type Output = TokenIdxRange;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.raw()]
    }
}

impl std::ops::Index<ExprIdx> for ExprRangeRegion {
    type Output = TokenIdxRange;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_ranges[index.raw()]
    }
}

struct ExprRangeCalculator<'a> {
    token_sheet_data: &'a TokenSheetData,
    expr_region_data: &'a ExprRegionData,
    entity_path_expr_ranges: Vec<TokenIdxRange>,
    pattern_expr_ranges: Vec<TokenIdxRange>,
    expr_ranges: Vec<TokenIdxRange>,
    stmt_ranges: StmtMap<TokenIdxRange>,
}

impl<'a> std::ops::Index<EntityPathExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: EntityPathExprIdx) -> &Self::Output {
        &self.entity_path_expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<&EntityPathExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: &EntityPathExprIdx) -> &Self::Output {
        &self.entity_path_expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<PatternExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<ExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<&ExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: &ExprIdx) -> &Self::Output {
        &self.expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<StmtIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: StmtIdx) -> &Self::Output {
        &self.stmt_ranges[index]
    }
}

impl<'a> std::ops::Index<&StmtIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: &StmtIdx) -> &Self::Output {
        &self.stmt_ranges[index]
    }
}

impl<'a> ExprRangeCalculator<'a> {
    fn new(db: &'a dyn ExprDb, expr_region: ExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        let path = expr_region_data.path();
        let token_sheet_data = db.token_sheet_data(path.module_path(db)).unwrap();
        ExprRangeCalculator {
            token_sheet_data,
            expr_region_data,
            entity_path_expr_ranges: Default::default(),
            pattern_expr_ranges: Default::default(),
            expr_ranges: Default::default(),
            stmt_ranges: StmtMap::new(expr_region_data.stmt_arena()),
        }
    }

    fn calc_all(mut self) -> ExprRangeRegion {
        // order matters
        self.entity_path_expr_ranges
            .reserve(self.expr_region_data.entity_path_expr_arena().len());
        for entity_path_expr in self.expr_region_data.entity_path_expr_arena().iter() {
            self.entity_path_expr_ranges
                .push(self.calc_entity_path_expr_range(entity_path_expr))
        }
        self.pattern_expr_ranges
            .reserve(self.expr_region_data.pattern_expr_arena().len());
        for pattern_expr in self.expr_region_data.pattern_expr_arena().iter() {
            self.pattern_expr_ranges
                .push(self.calc_pattern_expr_range(pattern_expr))
        }
        self.expr_ranges
            .reserve(self.expr_region_data.expr_arena().len());
        for expr in self.expr_region_data.expr_arena().iter() {
            let expr_range = self.calc_expr_range(expr);
            self.expr_ranges.push(expr_range)
        }
        assert_eq!(
            self.expr_region_data.expr_arena().len(),
            self.expr_ranges.len()
        );
        ExprRangeRegion {
            entity_path_expr_ranges: self.entity_path_expr_ranges,
            pattern_expr_ranges: self.pattern_expr_ranges,
            expr_ranges: self.expr_ranges,
            stmt_ranges: self.stmt_ranges,
        }
    }

    fn calc_entity_path_expr_range(&self, expr: &EntityPathExpr) -> TokenIdxRange {
        match expr {
            EntityPathExpr::Root {
                token_idx,
                ident,
                entity_path,
            } => TokenIdxRange::new_single(*token_idx),
            EntityPathExpr::Subentity {
                parent,
                scope_resolution_token,
                ident_token,
                path,
            } => match ident_token {
                Ok(ident_token) => {
                    self[parent].to(TokenIdxRangeEnd::new_after(ident_token.token_idx()))
                }
                Err(_) => self[parent].to(TokenIdxRangeEnd::new_after(
                    scope_resolution_token.token_idx(),
                )),
            },
        }
    }

    fn calc_pattern_expr_range(&self, expr: &PatternExpr) -> TokenIdxRange {
        match expr {
            PatternExpr::Literal(_) => todo!(),
            PatternExpr::Ident {
                modifier_keyword_group,
                ident_token,
            } => match modifier_keyword_group {
                Some(modifier_keyword_group) => match modifier_keyword_group {
                    PatternSymbolModifierKeywordGroup::Mut(mut_token) => {
                        TokenIdxRange::new_closed(mut_token.token_idx(), ident_token.token_idx())
                    }
                    PatternSymbolModifierKeywordGroup::RefMut(ref_token, _) => {
                        TokenIdxRange::new_closed(ref_token.token_idx(), ident_token.token_idx())
                    }
                },
                None => TokenIdxRange::new_single(ident_token.token_idx()),
            },
            PatternExpr::Entity(_) => todo!(),
            PatternExpr::Tuple { name, fields } => todo!(),
            PatternExpr::Struct { name, fields } => todo!(),
            PatternExpr::OneOf { options } => todo!(),
            PatternExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            PatternExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    fn calc_expr_range(&mut self, expr: &Expr) -> TokenIdxRange {
        match expr {
            Expr::Literal(token_idx)
            | Expr::InheritedSymbol { token_idx, .. }
            | Expr::CurrentSymbol { token_idx, .. }
            | Expr::FrameVarDecl { token_idx, .. }
            | Expr::SelfType(token_idx)
            | Expr::SelfValue(token_idx) => TokenIdxRange::new_single(*token_idx),
            Expr::Binary { lopd, ropd, .. } => self[lopd].join(self[ropd]),
            Expr::EntityPath {
                entity_path_expr,
                path: entity_path,
            } => self[*entity_path_expr],
            Expr::Be {
                src,
                be_token_idx,
                target,
            } => {
                let start = self[src].start().token_idx();
                let end = if let Ok(target) = target {
                    self[target.pattern_expr()].end()
                } else {
                    TokenIdxRangeEnd::new_after(*be_token_idx)
                };
                TokenIdxRange::new(start, end)
            }
            Expr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => TokenIdxRange::new(*opr_token_idx, self[opd].end()),
            Expr::Suffix {
                opd,
                opr,
                opr_token_idx,
            } => self[opd].to(TokenIdxRangeEnd::new_after(*opr_token_idx)),
            Expr::ExplicitApplicationOrRitchieCall {
                function: first_expr,
                rpar_token_idx,
                ..
            }
            | Expr::MethodCall {
                self_argument: first_expr,
                rpar_token_idx,
                ..
            } => self[first_expr].to(TokenIdxRangeEnd::new_after(*rpar_token_idx)),
            Expr::Field {
                owner, ident_token, ..
            } => self[owner].to(TokenIdxRangeEnd::new_after(ident_token.token_idx())),
            Expr::TemplateInstantiation {
                template,
                implicit_arguments,
            } => todo!(),
            Expr::ExplicitApplication { function, argument } => self[function].join(self[argument]),
            Expr::Unit {
                lpar_token_idx,
                rpar_token_idx,
                ..
            }
            | Expr::Bracketed {
                lpar_token_idx,
                rpar_token_idx,
                ..
            }
            | Expr::NewTuple {
                lpar_token_idx,
                rpar_token_idx,
                ..
            } => TokenIdxRange::new(
                *lpar_token_idx,
                TokenIdxRangeEnd::new_after(*rpar_token_idx),
            ),
            Expr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                rbox_token_idx,
                ..
            } => self[owner].to(TokenIdxRangeEnd::new_after(*rbox_token_idx)),
            Expr::List {
                lbox_token_idx,
                rbox_token_idx,
                ..
            } => TokenIdxRange::new(
                *lbox_token_idx,
                TokenIdxRangeEnd::new_after(*rbox_token_idx),
            ),
            Expr::BoxColonList {
                lbox_token_idx,
                rbox_token_idx,
                ..
            } => TokenIdxRange::new(
                *lbox_token_idx,
                TokenIdxRangeEnd::new_after(*rbox_token_idx),
            ),
            Expr::Block { stmts } => self.calc_block_range(*stmts),
            Expr::EmptyHtmlTag {
                empty_html_bra_idx,
                empty_html_ket,
                ..
            } => TokenIdxRange::new_closed(*empty_html_bra_idx, empty_html_ket.token_idx()),
            Expr::Err(error) => match error {
                ExprError::Original(error) => match error {
                    OriginalExprError::MismatchingBracket {
                        ket_token_idx: token_idx,
                        ..
                    }
                    | OriginalExprError::ExpectRightAngleBracket {
                        langle_token_idx: token_idx,
                    }
                    | OriginalExprError::ExpectRightCurlyBrace(token_idx)
                    | OriginalExprError::ExpectIdent(token_idx)
                    | OriginalExprError::ExpectColon(token_idx)
                    | OriginalExprError::ExpectRightParenthesis(token_idx)
                    | OriginalExprError::NoMatchingBra {
                        ket_token_idx: token_idx,
                        ..
                    }
                    | OriginalExprError::ExpectIdentAfterDot(token_idx)
                    | OriginalExprError::NoLeftOperandForBinaryOperator {
                        binary_token_idx: token_idx,
                    }
                    | OriginalExprError::NoRightOperandForBinaryOperator {
                        punctuation_token_idx: token_idx,
                        ..
                    }
                    | OriginalExprError::NoOperandForPrefixOperator {
                        prefix_token_idx: token_idx,
                        ..
                    }
                    | OriginalExprError::ExpectItemBeforeComma {
                        comma_token_idx: token_idx,
                    }
                    | OriginalExprError::ExpectItemBeforeBe {
                        be_token_idx: token_idx,
                    }
                    | OriginalExprError::ExpectedLetVariablesPattern(token_idx)
                    | OriginalExprError::ExpectedBeVariablesPattern(token_idx)
                    | OriginalExprError::ExpectedLetVariablesType(token_idx)
                    | OriginalExprError::ExpectedAssign(token_idx)
                    | OriginalExprError::ExpectedInitialValue(token_idx)
                    | OriginalExprError::UnexpectedKeyword(token_idx)
                    | OriginalExprError::ExpectedResult(token_idx)
                    | OriginalExprError::ExpectedCondition(token_idx)
                    | OriginalExprError::ExpectedForExpr(token_idx)
                    | OriginalExprError::ExpectedBePattern(token_idx)
                    | OriginalExprError::ExpectedParameterPattern(token_idx)
                    | OriginalExprError::UnterminatedList {
                        bra_token_idx: token_idx,
                    }
                    | OriginalExprError::ExpectedEolColon(token_idx)
                    | OriginalExprError::ExpectedIdentAfterModifier(token_idx)
                    | OriginalExprError::UnexpectedSheba(token_idx)
                    | OriginalExprError::UnrecognizedIdent { token_idx, .. }
                    | OriginalExprError::UnresolvedSubentity { token_idx, .. }
                    | OriginalExprError::ExpectedFieldType(token_idx)
                    | OriginalExprError::ExpectedParameterType(token_idx)
                    | OriginalExprError::SelfTypeNotAllowed(token_idx)
                    | OriginalExprError::SelfValueNotAllowed(token_idx)
                    | OriginalExprError::ExpectedIdentAfterDot {
                        dot_token_idx: token_idx,
                        ..
                    }
                    | OriginalExprError::ExpectedExprBeforeDot {
                        dot_token_idx: token_idx,
                    }
                    | OriginalExprError::HtmlTodo(token_idx) => {
                        TokenIdxRange::new_single(*token_idx)
                    }
                    OriginalExprError::ExpectBlock(_) => todo!(),
                },
                ExprError::Derived(_) => todo!(),
            },
        }
    }

    fn calc_block_range(&mut self, stmts: StmtIdxRange) -> TokenIdxRange {
        for stmt in stmts {
            self.save_stmt_range(stmt);
        }
        self[stmts.start()].join(self[stmts.end() - 1])
    }

    fn save_stmt_range(&mut self, stmt_idx: StmtIdx) {
        let range = self.calc_stmt_range(stmt_idx);
        // after calculation, all the child statements must have already been computed and cached
        // so that self.stmt_ranges.len() is equal to stmt_idx.raw()
        self.stmt_ranges.insert_new(stmt_idx, range)
    }

    fn calc_stmt_range(&mut self, stmt_idx: StmtIdx) -> TokenIdxRange {
        match self.expr_region_data[stmt_idx] {
            Stmt::Let {
                let_token,
                ref let_variable_pattern,
                ref assign_token,
                ref initial_value, /* todo: other types of let initialization */
                ..
            } => {
                let start = let_token.token_idx();
                let end = if let Ok(initial_value) = initial_value {
                    self[initial_value].end()
                } else if let Ok(assign_token) = assign_token {
                    TokenIdxRangeEnd::new_after(assign_token.token_idx())
                } else if let Ok(let_variable_pattern) = let_variable_pattern {
                    todo!()
                } else {
                    TokenIdxRangeEnd::new_after(let_token.token_idx())
                };
                TokenIdxRange::new(start, end)
            }
            Stmt::Return {
                return_token,
                ref result,
            } => {
                let start = return_token.token_idx();
                let end = if let Ok(result) = result {
                    self[result].end()
                } else {
                    TokenIdxRangeEnd::new_after(start)
                };
                TokenIdxRange::new(start, end)
            }
            Stmt::Require {
                require_token,
                ref condition,
            } => {
                let start = require_token.token_idx();
                let end = if let Ok(condition) = condition {
                    self[condition].end()
                } else {
                    TokenIdxRangeEnd::new_after(start)
                };
                TokenIdxRange::new(start, end)
            }
            Stmt::Assert {
                assert_token,
                ref condition,
            } => {
                let start = assert_token.token_idx();
                let end = if let Ok(condition) = condition {
                    self[condition].end()
                } else {
                    TokenIdxRangeEnd::new_after(start)
                };
                TokenIdxRange::new(start, end)
            }
            Stmt::Break { break_token } => TokenIdxRange::new_single(break_token.token_idx()),
            Stmt::Eval { expr_idx } => self[expr_idx],
            Stmt::ForBetween {
                for_token,
                ref particulars,
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = for_token.token_idx();
                let end = if let Ok(block) = block {
                    self.calc_block_range(*block).end()
                } else if let Ok(eol_colon) = eol_colon {
                    TokenIdxRangeEnd::new_after(eol_colon.token_idx())
                } else if let Some(bound_expr) = particulars.range.final_boundary.bound_expr {
                    self[bound_expr].end()
                } else {
                    TokenIdxRangeEnd::new_after(particulars.frame_var_token_idx)
                };
                TokenIdxRange::new(start, end)
            }
            Stmt::ForIn {
                for_token,
                ref block,
                ..
            } => todo!(),
            Stmt::ForExt {
                forext_token,
                /* todo: particulars */
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = forext_token.token_idx();
                let end = if let Ok(block) = block {
                    self.calc_block_range(*block).end()
                } else if let Ok(eol_colon) = eol_colon {
                    TokenIdxRangeEnd::new_after(eol_colon.token_idx())
                }
                /* todo: particulars */
                else {
                    TokenIdxRangeEnd::new_after(start)
                };
                TokenIdxRange::new(start, end)
            }
            Stmt::While {
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = while_token.token_idx();
                let end = if let Ok(block) = block {
                    self.calc_block_range(*block).end()
                } else if let Ok(eol_colon) = eol_colon {
                    TokenIdxRangeEnd::new_after(eol_colon.token_idx())
                } else if let Ok(condition) = condition {
                    self[condition].end()
                } else {
                    TokenIdxRangeEnd::new_after(start)
                };
                TokenIdxRange::new(start, end)
            }
            Stmt::DoWhile {
                do_token,
                ref condition,
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = do_token.token_idx();
                let end = if let Ok(block) = block {
                    self.calc_block_range(*block).end()
                } else if let Ok(eol_colon) = eol_colon {
                    TokenIdxRangeEnd::new_after(eol_colon.token_idx())
                } else if let Ok(condition) = condition {
                    self[condition].end()
                } else {
                    TokenIdxRangeEnd::new_after(start)
                };
                TokenIdxRange::new(start, end)
            }
            Stmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                let start = if_branch.if_token.token_idx();
                // it's important that every branch is computed
                let if_branch_end: TokenIdxRangeEnd = if let Ok(block) = if_branch.block() {
                    self.calc_block_range(block).end()
                } else if let Ok(eol_colon_token) = if_branch.eol_colon_token() {
                    TokenIdxRangeEnd::new_after(eol_colon_token.token_idx())
                } else if let Ok(condition) = if_branch.condition {
                    self[condition].end()
                } else {
                    TokenIdxRangeEnd::new_after(if_branch.if_token.token_idx())
                };
                let mut elif_branch_rev_iter = elif_branches.iter().rev();
                let elif_branches_end: Option<TokenIdxRangeEnd> = {
                    if let Some(last_elif_branch) = elif_branch_rev_iter.next() {
                        if let Ok(block) = last_elif_branch.block() {
                            Some(self.calc_block_range(block).end())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };
                for elif_branch in elif_branch_rev_iter {
                    if let Ok(block) = elif_branch.block() {
                        self.calc_block_range(block);
                    }
                }
                let else_block_end: Option<TokenIdxRangeEnd> =
                    if let Some(else_branch) = else_branch {
                        if let Ok(block) = else_branch.block() {
                            Some(self.calc_block_range(block).end())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                let end = else_block_end
                    .or(elif_branches_end)
                    .unwrap_or(if_branch_end);
                TokenIdxRange::new(start, end)
            }
            Stmt::Match { match_token } => {
                // ad hoc
                TokenIdxRange::new_single(match_token.token_idx())
            }
            Stmt::Err(_) => todo!(),
        }
    }
}
