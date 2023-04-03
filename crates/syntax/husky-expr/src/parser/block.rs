use super::*;
use husky_ast::{AstIdx, AstTokenIdxRangeSheet, FormBody};

pub struct BlockExprParser<'a> {
    expr_parser: ExprParser<'a>,
    ast_sheet: &'a AstSheet,
    ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
    env: Option<ExprEnvironment>,
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
        env: Option<ExprEnvironment>,
    ) -> Self {
        Self {
            expr_parser,
            ast_sheet,
            ast_token_idx_range_sheet,
            env,
        }
    }

    pub fn ast_sheet(&self) -> &'a AstSheet {
        self.ast_sheet
    }

    fn ctx<'b>(&'b mut self, token_stream: TokenStream<'a>) -> ExprParseContext<'a, 'b>
    where
        'a: 'b,
    {
        let env = self.env;
        ExprParseContext::new(self, env, token_stream)
    }

    pub fn parse_block_stmts_expected(
        &mut self,
        body: FormBody,
        token_group_idx: TokenGroupIdx,
    ) -> ExprResult<StmtIdxRange> {
        match self.parse_block_stmts(body) {
            Some(stmt_idx_range) => Ok(stmt_idx_range),
            None => Err(OriginalExprError::ExpectBlock(token_group_idx).into()),
        }
    }

    pub fn parse_block_stmts(&mut self, body: FormBody) -> Option<StmtIdxRange> {
        let block_end = self.form_body_end(body);
        let body = body.ast_idx_range();
        if body.len() == 0 {
            return None;
        }
        let stmts = self
            .ast_sheet
            .indexed_iter(body)
            .filter_map(|(idx, ast)| {
                self.parse_stmt(ast, self.ast_token_idx_range_sheet[idx], block_end)
            })
            .collect();
        Some(self.alloc_stmts(stmts))
    }

    pub fn parse_block_expr(&mut self, body: FormBody) -> ExprIdx {
        let stmts = self
            .parse_block_stmts(body)
            .expect("husky-ast should guarantee that this not empty");
        let expr = self.alloc_expr(Expr::Block { stmts });
        self.expr_roots
            .push(ExprRoot::new(ExprRootKind::BlockExpr, expr));
        expr
    }

    fn parse_stmt(
        &mut self,
        ast: &Ast,
        ast_token_idx_range: TokenIdxRange,
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
                token_group_idx,
                pattern_stmt,
                case_stmts,
                ..
            } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(*token_group_idx, None);
                Some(Stmt::Match {
                    match_token: token_stream.parse().unwrap().unwrap(),
                })
            }
            Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Attr { .. }
            | Ast::Decr { .. }
            | Ast::Defn { .. }
            | Ast::TypeVariant { .. }
            | Ast::ImplBlock { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => None,
        }
    }

    fn parse_basic_stmt(
        &mut self,
        token_group_idx: TokenGroupIdx,
        block_end: TokenIdxRangeEnd,
        body: Option<FormBody>,
    ) -> Option<Stmt> {
        let token_stream = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, None);
        let mut ctx = self.ctx(token_stream);
        match ctx.parse::<BasicStmtKeywordToken>() {
            Ok(Some(basic_stmt_keyword_token)) => Some(match basic_stmt_keyword_token {
                BasicStmtKeywordToken::Let(let_token) => Stmt::Let {
                    let_token,
                    let_variable_pattern: ctx.parse_let_variables_pattern_expected(block_end),
                    assign_token: ctx.parse_expected(OriginalExprError::ExpectedAssign),
                    initial_value: ctx
                        .parse_expr_expected(None, OriginalExprError::ExpectedInitialValue),
                },
                BasicStmtKeywordToken::Return(return_token) => Stmt::Return {
                    return_token,
                    result: ctx.parse_expr_expected(None, OriginalExprError::ExpectedResult),
                },
                BasicStmtKeywordToken::Require(require_token) => Stmt::Require {
                    require_token,
                    condition: ctx.parse_expr_expected(
                        Some(ExprEnvironment::Condition(block_end)),
                        OriginalExprError::ExpectedCondition,
                    ),
                },
                BasicStmtKeywordToken::Assert(assert_token) => Stmt::Assert {
                    assert_token,
                    condition: ctx.parse_expr_expected(
                        Some(ExprEnvironment::Condition(block_end)),
                        OriginalExprError::ExpectedCondition,
                    ),
                },
                BasicStmtKeywordToken::Break(break_token) => Stmt::Break { break_token },
                BasicStmtKeywordToken::For(for_token) => {
                    let expr =
                        match ctx.parse_expr_expected(None, OriginalExprError::ExpectedCondition) {
                            Ok(expr) => expr,
                            Err(_) => todo!(),
                        };
                    let eol_colon = ctx.parse_expected(OriginalExprError::ExpectedEolColon);
                    self.parse_for_loop_stmt(
                        expr,
                        for_token,
                        eol_colon,
                        token_group_idx,
                        body.expect("should be checked in `husky_ast`"),
                    )
                    .into()
                }
                BasicStmtKeywordToken::ForExt(forext_token) => Stmt::ForExt {
                    forext_token,
                    // condition: ctx
                    //     .parse_expr(ExprParseEnvironment::None)
                    //     .ok_or(ExprError::ExpectCondition),
                    eol_colon: ctx.parse_expected(OriginalExprError::ExpectedEolColon),
                    block: self.parse_block_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                },
                BasicStmtKeywordToken::While(while_token) => Stmt::While {
                    while_token,
                    condition: ctx.parse_expr_expected(
                        Some(ExprEnvironment::Condition(block_end)),
                        OriginalExprError::ExpectedCondition,
                    ),
                    eol_colon: ctx.parse_expected(OriginalExprError::ExpectedEolColon),
                    block: self.parse_block_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                },
                BasicStmtKeywordToken::Do(do_token) => match ctx.parse::<WhileToken>() {
                    Ok(Some(while_token)) => Stmt::DoWhile {
                        do_token,
                        while_token,
                        condition: ctx.parse_expr_expected(
                            Some(ExprEnvironment::Condition(block_end)),
                            OriginalExprError::ExpectedCondition,
                        ),
                        eol_colon: ctx.parse_expected(OriginalExprError::ExpectedEolColon),
                        block: self.parse_block_stmts_expected(
                            body.expect("should be checked in `husky_ast`"),
                            token_group_idx,
                        ),
                    },
                    Ok(None) => todo!(),
                    Err(_) => todo!(),
                },
            }),
            Ok(None) => ctx
                .parse_expr(None)
                .map(|expr| Stmt::Eval { expr_idx: expr }),
            Err(_) => todo!(),
        }
    }

    fn parse_for_loop_stmt(
        &mut self,
        expr: ExprIdx,
        for_token: StmtForToken,
        eol_colon: ExprResult<EolToken>,
        token_group_idx: TokenGroupIdx,
        body: FormBody,
    ) -> StmtResult<Stmt> {
        match self.expr_arena[expr] {
            Expr::Binary {
                lopd,
                opr: BinaryOpr::Comparison(comparison_opr),
                opr_token_idx,
                ropd,
            } => {
                let particulars = self.parse_for_between_particulars(lopd, ropd, comparison_opr)?;
                let current_symbol_variant = CurrentSymbolVariant::FrameVariable {
                    expr_idx: particulars.frame_var_expr_idx,
                    ident: particulars.frame_var_ident,
                };
                let current_symbol_kind = current_symbol_variant.kind();
                let access_start = self.ast_token_idx_range_sheet[body.ast_idx_range().start()]
                    .start()
                    .token_idx();
                let access_end =
                    self.ast_token_idx_range_sheet[body.ast_idx_range().end() - 1].end();
                let frame_var_symbol =
                    CurrentSymbol::new(access_start, Some(access_end), current_symbol_variant);
                let frame_var_symbol_idx = self
                    .define_symbols(
                        vec![frame_var_symbol],
                        Some(PatternTypeConstraint::FrameVariable),
                    )
                    .start();
                self.expr_arena.set(
                    particulars.frame_var_expr_idx,
                    Expr::FrameVarDecl {
                        token_idx: particulars.frame_var_token_idx,
                        ident: particulars.frame_var_ident,
                        frame_var_symbol_idx,
                        current_symbol_kind,
                    },
                );
                Ok(Stmt::ForBetween {
                    for_token,
                    particulars,
                    frame_var_symbol_idx,
                    eol_colon,
                    block: self.parse_block_stmts_expected(body, token_group_idx),
                })
            }
            Expr::Binary {
                lopd,
                opr: BinaryOpr::In,
                opr_token_idx,
                ropd,
            } => Ok(Stmt::ForIn {
                for_token,
                condition: todo!(),
                eol_colon,
                block: self.parse_block_stmts_expected(body, token_group_idx),
            }),
            _ => todo!(),
        }
    }

    fn parse_for_between_particulars(
        &self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        comparison_opr: BinaryComparisonOpr,
    ) -> Result<ForBetweenParticulars, StmtError> {
        use OriginalExprError::UnrecognizedIdent;
        let lopd_expr = &self.expr_arena[lopd];
        let ropd_expr = &self.expr_arena[ropd];
        // todo: parse with
        if let Expr::Err(ExprError::Original(UnrecognizedIdent { token_idx, ident })) = lopd_expr {
            Ok(ForBetweenParticulars {
                frame_var_token_idx: *token_idx,
                frame_var_expr_idx: lopd,
                frame_var_ident: *ident,
                range: ForBetweenRange::new_with_default_initial(comparison_opr, ropd)?,
            })
        } else if let Expr::Err(ExprError::Original(UnrecognizedIdent { token_idx, ident })) =
            ropd_expr
        {
            Ok(ForBetweenParticulars {
                frame_var_token_idx: *token_idx,
                frame_var_expr_idx: ropd,
                frame_var_ident: *ident,
                range: ForBetweenRange::new_with_default_final(lopd, comparison_opr)?,
            })
        } else {
            let final_comparison = comparison_opr;
            match lopd_expr {
                Expr::Binary {
                    lopd: llopd,
                    opr: BinaryOpr::Comparison(initial_comparison),
                    opr_token_idx,
                    ropd: lropd,
                } => {
                    let lropd_expr = &self.expr_arena[lropd];
                    match lropd_expr {
                        Expr::Err(ExprError::Original(UnrecognizedIdent { token_idx, ident })) => {
                            Ok(ForBetweenParticulars {
                                frame_var_token_idx: *token_idx,
                                frame_var_expr_idx: *lropd,
                                frame_var_ident: *ident,
                                range: ForBetweenRange::new_without_defaults(
                                    *llopd,
                                    *initial_comparison,
                                    final_comparison,
                                    ropd,
                                )?,
                            })
                        }
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
    }

    fn parse_if_branch(&mut self, if_branch: AstIdx) -> IfBranch {
        match self.ast_sheet[if_branch] {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => {
                let body_end = self.form_body_end(body.expect("should be checked in `husky_ast`"));
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(token_group_idx, None);
                let mut ctx = self.ctx(token_stream);
                IfBranch {
                    if_token: ctx.parse().unwrap().unwrap(),
                    condition: ctx.parse_expr_expected(
                        Some(ExprEnvironment::Condition(body_end)),
                        OriginalExprError::ExpectedCondition,
                    ),
                    eol_colon: ctx.parse_expected(OriginalExprError::ExpectedEolColon),
                    block: self.parse_block_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                }
            }
            _ => unreachable!(),
        }
    }
    fn form_body_end(&self, body: FormBody) -> TokenIdxRangeEnd {
        self.ast_token_idx_range_sheet[body.ast_idx_range().end() - 1].end()
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
                body,
            } => {
                let body = body.expect("should be checked in `husky_ast`");
                let body_end = self.form_body_end(body);
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(token_group_idx, None);
                let mut ctx = self.ctx(token_stream);
                ElifBranch {
                    elif_token: ctx.parse().unwrap().unwrap(),
                    condition: ctx.parse_expr_expected(
                        Some(ExprEnvironment::Condition(body_end)),
                        OriginalExprError::ExpectedCondition,
                    ),
                    eol_colon: ctx.parse_expected(OriginalExprError::ExpectedEolColon),
                    block: self.parse_block_stmts_expected(body, token_group_idx),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_else_branch(&mut self, else_branch: Option<AstIdx>) -> Option<ElseBranch> {
        match self.ast_sheet[else_branch?] {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(token_group_idx, None);
                let mut ctx = self.ctx(token_stream);
                Some(ElseBranch {
                    else_token: ctx.parse().unwrap().unwrap(),
                    eol_colon: ctx.parse_expected(OriginalExprError::ExpectedEolColon),
                    block: self.parse_block_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                })
            }
            _ => unreachable!(),
        }
    }

    pub fn finish(self) -> ExprRegion {
        self.expr_parser.finish()
    }
}
