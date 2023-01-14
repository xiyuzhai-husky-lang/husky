use super::*;
use husky_ast::{AstIdx, AstTokenIdxRangeSheet};

pub struct BlockExprParser<'a> {
    expr_parser: ExprParser<'a>,
    ast_sheet: &'a AstSheet,
    ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
}

impl<'a> std::ops::Deref for BlockExprParser<'a> {
    type Target = ExprParser<'a>;

    fn deref(&self) -> &Self::Target {
        &self.expr_parser
    }
}

impl<'a> std::ops::DerefMut for BlockExprParser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.expr_parser
    }
}

impl<'a> BlockExprParser<'a> {
    pub fn new(
        expr_parser: ExprParser<'a>,
        ast_sheet: &'a AstSheet,
        ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
    ) -> Self {
        Self {
            expr_parser,
            ast_sheet,
            ast_token_idx_range_sheet,
        }
    }

    pub fn parse_block_stmts_expected(
        &mut self,
        body: AstIdxRange,
        token_group_idx: TokenGroupIdx,
    ) -> ExprResult<StmtIdxRange> {
        match self.parse_block_stmts(body) {
            Some(stmt_idx_range) => Ok(stmt_idx_range),
            None => Err(ExprError::MissingBlock(token_group_idx)),
        }
    }

    pub fn parse_block_stmts(&mut self, body: AstIdxRange) -> Option<StmtIdxRange> {
        if body.len() == 0 {
            return None;
        }
        let block_end = self.ast_token_idx_range_sheet[body.end() - 1].end();
        let stmts = self
            .ast_sheet
            .indexed_iter(body)
            .filter_map(|(idx, ast)| {
                self.parse_stmt(ast, self.ast_token_idx_range_sheet[idx], block_end)
            })
            .collect();
        Some(self.alloc_stmts(stmts))
    }

    pub fn parse_block_expr(&mut self, body: AstIdxRange) -> Option<ExprIdx> {
        let stmts = self.parse_block_stmts(body)?;
        Some(self.alloc_expr(Expr::Block { stmts }))
    }

    fn parse_stmt(
        &mut self,
        ast: &Ast,
        ast_range: TokenIdxRange,
        block_end: TokenIdxRangeEnd,
    ) -> Option<Stmt> {
        match ast {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => self.parse_basic_stmt(*token_group_idx, block_end, *body),
            Ast::IfElseStmts {
                if_branch,
                elif_branches,
                else_branch,
            } => Some(Stmt::IfElse {
                if_branch: self.parse_if_branch(*if_branch),
                elif_branches: self.parse_elif_branches(*elif_branches),
                else_branch: self.parse_else_branch(*else_branch),
            }),
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
            } => Some(Stmt::Match {}),
            Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Decor { .. }
            | Ast::Defn { .. }
            | Ast::ModuleItemVariant { .. }
            | Ast::Impl { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => None,
        }
    }

    fn parse_basic_stmt(
        &mut self,
        token_group_idx: TokenGroupIdx,
        block_end: TokenIdxRangeEnd,
        body: AstIdxRange,
    ) -> Option<Stmt> {
        let token_stream = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, None);
        let mut ctx = self.ctx(token_stream);
        match ctx.parse::<BasicStmtKeywordToken>() {
            Ok(Some(basic_stmt_keyword_token)) => Some(match basic_stmt_keyword_token {
                BasicStmtKeywordToken::Let(let_token) => Stmt::Let {
                    let_token,
                    let_variable_pattern: ctx.parse_let_variable_pattern(block_end),
                    assign_token: ctx.parse_expected(),
                    initial_value: ctx.parse_expr_expected(
                        ExprParseEnvironment::None,
                        ExprError::MissingInitialValue,
                    ),
                },
                BasicStmtKeywordToken::Return(return_token) => Stmt::Return {
                    return_token,
                    result: ctx
                        .parse_expr_expected(ExprParseEnvironment::None, ExprError::MissingResult),
                },
                BasicStmtKeywordToken::Require(require_token) => Stmt::Require {
                    require_token,
                    condition: ctx.parse_expr_expected(
                        ExprParseEnvironment::None,
                        ExprError::MissingCondition,
                    ),
                },
                BasicStmtKeywordToken::Assert(assert_token) => Stmt::Assert {
                    assert_token,
                    condition: ctx.parse_expr_expected(
                        ExprParseEnvironment::None,
                        ExprError::MissingCondition,
                    ),
                },
                BasicStmtKeywordToken::Break(break_token) => Stmt::Break { break_token },
                BasicStmtKeywordToken::For(for_token) => {
                    let expr = match ctx.parse_expr_expected(
                        ExprParseEnvironment::None,
                        ExprError::MissingCondition,
                    ) {
                        Ok(expr) => expr,
                        Err(_) => todo!(),
                    };
                    let eol_colon = ctx.parse_expected();
                    let block = self.parse_block_stmts_expected(body, token_group_idx);
                    self.parse_for_loop(expr, for_token, eol_colon, block)
                }
                BasicStmtKeywordToken::ForExt(forext_token) => Stmt::ForExt {
                    forext_token,
                    // condition: ctx
                    //     .parse_expr(ExprParseEnvironment::None)
                    //     .ok_or(ExprError::MissingCondition),
                    eol_colon: ctx.parse_expected(),
                    block: self.parse_block_stmts_expected(body, token_group_idx),
                },
                BasicStmtKeywordToken::While(while_token) => Stmt::While {
                    while_token,
                    condition: ctx.parse_expr_expected(
                        ExprParseEnvironment::None,
                        ExprError::MissingCondition,
                    ),
                    eol_colon: ctx.parse_expected(),
                    block: self.parse_block_stmts_expected(body, token_group_idx),
                },
                BasicStmtKeywordToken::Do(do_token) => match ctx.parse::<WhileToken>() {
                    Ok(Some(while_token)) => Stmt::DoWhile {
                        do_token,
                        while_token,
                        condition: ctx.parse_expr_expected(
                            ExprParseEnvironment::None,
                            ExprError::MissingCondition,
                        ),
                        eol_colon: ctx.parse_expected(),
                        block: self.parse_block_stmts_expected(body, token_group_idx),
                    },
                    Ok(None) => todo!(),
                    Err(_) => todo!(),
                },
            }),
            Ok(None) => ctx
                .parse_expr(ExprParseEnvironment::None)
                .map(|expr| Stmt::Eval { expr }),
            Err(_) => todo!(),
        }
    }

    fn parse_for_loop(
        &mut self,
        expr: ExprIdx,
        for_token: ForToken,
        eol_colon: ExprResult<EolColonToken>,
        block: ExprResult<StmtIdxRange>,
    ) -> Stmt {
        match self.expr_arena[expr] {
            Expr::BinaryOpn {
                lopd,
                opr: BinaryOpr::Comparison(_),
                opr_token_idx,
                ropd,
            } => Stmt::ForBetween {
                for_token,
                frame_var_ident: todo!(),
                frame_var_token_idx: todo!(),
                initial_boundary: todo!(),
                final_boundary: todo!(),
                step: todo!(),
                eol_colon,
                block,
            },
            Expr::BinaryOpn {
                lopd,
                opr: BinaryOpr::In,
                opr_token_idx,
                ropd,
            } => Stmt::ForIn {
                for_token,
                condition: todo!(),
                eol_colon,
                block,
            },
            _ => todo!(),
        }
    }

    fn parse_if_branch(&mut self, if_branch: AstIdx) -> IfBranch {
        match self.ast_sheet[if_branch] {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                ref body,
            } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(token_group_idx, None);
                let mut ctx = self.ctx(token_stream);
                IfBranch {
                    if_token: ctx.parse().unwrap().unwrap(),
                    condition: ctx.parse_expr_expected(
                        ExprParseEnvironment::None,
                        ExprError::MissingCondition,
                    ),
                    eol_colon: ctx.parse_expected(),
                    block: self.parse_block_stmts_expected(*body, token_group_idx),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_elif_branches(&mut self, elif_branches: AstIdxRange) -> Vec<ElifBranch> {
        elif_branches
            .into_iter()
            .map(|elif_branch| self.parse_elif_branch(elif_branch))
            .collect()
    }

    fn parse_elif_branch(&mut self, elif_branch: AstIdx) -> ElifBranch {
        match self.ast_sheet[elif_branch] {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                ref body,
            } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(token_group_idx, None);
                let mut ctx = self.ctx(token_stream);
                ElifBranch {
                    elif_token: ctx.parse().unwrap().unwrap(),
                    condition: ctx.parse_expr_expected(
                        ExprParseEnvironment::None,
                        ExprError::MissingCondition,
                    ),
                    eol_colon: ctx.parse_expected(),
                    block: self.parse_block_stmts_expected(*body, token_group_idx),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_else_branch(&mut self, else_branch: Option<AstIdx>) -> Option<ElseBranch> {
        match self.ast_sheet[else_branch?] {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                ref body,
            } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(token_group_idx, None);
                let mut ctx = self.ctx(token_stream);
                Some(ElseBranch {
                    else_token: ctx.parse().unwrap().unwrap(),
                    eol_colon: ctx.parse_expected(),
                    block: self.parse_block_stmts_expected(*body, token_group_idx),
                })
            }
            _ => unreachable!(),
        }
    }

    pub fn finish(self) -> ExprSheet {
        self.expr_parser.finish()
    }
}
