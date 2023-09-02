use super::*;
use husky_ast::{AstIdx, AstTokenIdxRangeSheet, FugitiveBody};

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
        body: FugitiveBody,
        token_group_idx: TokenGroupIdx,
    ) -> SynExprResult<SynStmtIdxRange> {
        match self.parse_block_stmts(body) {
            Some(stmt_idx_range) => Ok(stmt_idx_range),
            None => Err(OriginalExprError::ExpectedBlock(token_group_idx).into()),
        }
    }

    pub fn parse_block_stmts(&mut self, body: FugitiveBody) -> Option<SynStmtIdxRange> {
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

    pub fn parse_block_expr(&mut self, body: FugitiveBody) -> SynExprIdx {
        let stmts = self
            .parse_block_stmts(body)
            .expect("husky-ast should guarantee that this not empty");
        let expr = self.alloc_expr(SynExpr::Block { stmts });
        self.expr_parser
            .add_expr_root(ExprRootKind::BlockExpr, expr);
        expr
    }

    fn parse_stmt(
        &mut self,
        ast: &Ast,
        ast_token_idx_range: TokenIdxRange,
        block_end: TokenIdxRangeEnd,
    ) -> Option<SynStmt> {
        match ast {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => self.parse_basic_stmt(*token_group_idx, block_end, *body),
            Ast::IfElseStmts {
                if_branch,
                elif_branches,
                else_branch,
            } => Some(SynStmt::IfElse {
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
                Some(SynStmt::Match {
                    match_token: token_stream.try_parse_option().unwrap().unwrap(),
                })
            }
            Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Hint { .. }
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
        body: Option<FugitiveBody>,
    ) -> Option<SynStmt> {
        let token_stream = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, None);
        let mut ctx = self.ctx(token_stream);
        match ctx.try_parse_option::<BasicStmtKeywordToken>() {
            Ok(Some(basic_stmt_keyword_token)) => Some(match basic_stmt_keyword_token {
                BasicStmtKeywordToken::Let(let_token) => SynStmt::Let {
                    let_token,
                    let_variables_pattern: ctx.parse_let_variables_pattern_expected(block_end),
                    assign_token: ctx.try_parse_expected(OriginalExprError::ExpectedAssign),
                    initial_value: ctx.parse_expr_expected2(
                        None,
                        ExprRootKind::LetStmtInitialValue,
                        OriginalExprError::ExpectedInitialValue,
                    ),
                },
                BasicStmtKeywordToken::Return(return_token) => SynStmt::Return {
                    return_token,
                    result: ctx.parse_expr_expected2(
                        None,
                        ExprRootKind::ReturnExpr,
                        OriginalExprError::ExpectedResult,
                    ),
                },
                BasicStmtKeywordToken::Require(require_token) => SynStmt::Require {
                    require_token,
                    condition: ctx.parse_expr_expected2(
                        Some(ExprEnvironment::Condition(block_end)),
                        ExprRootKind::Condition,
                        OriginalExprError::ExpectedCondition,
                    ),
                },
                BasicStmtKeywordToken::Assert(assert_token) => SynStmt::Assert {
                    assert_token,
                    condition: ctx.parse_expr_expected2(
                        Some(ExprEnvironment::Condition(block_end)),
                        ExprRootKind::Condition,
                        OriginalExprError::ExpectedCondition,
                    ),
                },
                BasicStmtKeywordToken::Break(break_token) => SynStmt::Break { break_token },
                BasicStmtKeywordToken::For(for_token) => {
                    let expr =
                        match ctx.parse_expr_expected(None, OriginalExprError::ExpectedCondition) {
                            Ok(expr) => expr,
                            Err(_) => todo!(),
                        };
                    let eol_colon = ctx.try_parse_expected(OriginalExprError::ExpectedEolColon);
                    self.parse_for_loop_stmt(
                        token_group_idx,
                        for_token,
                        expr,
                        eol_colon,
                        body.expect("should be checked in `husky_ast`"),
                    )
                    .into()
                }
                BasicStmtKeywordToken::ForExt(forext_token) => {
                    let expr =
                        match ctx.parse_expr_expected(None, OriginalExprError::ExpectedCondition) {
                            Ok(expr) => expr,
                            Err(_) => todo!(),
                        };
                    let eol_colon = ctx.try_parse_expected(OriginalExprError::ExpectedEolColon);
                    self.parse_forext_loop_stmt(
                        token_group_idx,
                        forext_token,
                        expr,
                        eol_colon,
                        body.expect("should be checked in `husky_ast`"),
                    )
                    .into()
                }
                BasicStmtKeywordToken::While(while_token) => SynStmt::While {
                    while_token,
                    condition: ctx.parse_expr_expected(
                        Some(ExprEnvironment::Condition(block_end)),
                        OriginalExprError::ExpectedCondition,
                    ),
                    eol_colon: ctx.try_parse_expected(OriginalExprError::ExpectedEolColon),
                    block: self.parse_block_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                },
                BasicStmtKeywordToken::Do(do_token) => match ctx.try_parse_option::<WhileToken>() {
                    Ok(Some(while_token)) => SynStmt::DoWhile {
                        do_token,
                        while_token,
                        condition: ctx.parse_expr_expected(
                            Some(ExprEnvironment::Condition(block_end)),
                            OriginalExprError::ExpectedCondition,
                        ),
                        eol_colon: ctx.try_parse_expected(OriginalExprError::ExpectedEolColon),
                        block: self.parse_block_stmts_expected(
                            body.expect("should be checked in `husky_ast`"),
                            token_group_idx,
                        ),
                    },
                    Ok(None) => todo!(),
                    Err(_) => todo!(),
                },
            }),
            Ok(None) => match ctx.parse_expr_root(None, ExprRootKind::EvalExpr) {
                Some(expr_idx) => Some(SynStmt::Eval {
                    expr_idx,
                    eol_semicolon: ctx.try_parse_option(),
                }),
                None => None,
            },
            Err(_) => todo!(),
        }
    }

    fn parse_for_loop_stmt(
        &mut self,
        token_group_idx: TokenGroupIdx,
        for_token: StmtForToken,
        expr: SynExprIdx,
        eol_colon: SynExprResult<EolToken>,
        body: FugitiveBody,
    ) -> StmtResult<SynStmt> {
        match self.expr_arena[expr] {
            SynExpr::Binary {
                lopd,
                opr: BinaryOpr::Comparison(comparison_opr),
                opr_token_idx,
                ropd,
            } => {
                let particulars = self.parse_for_between_particulars(lopd, ropd, comparison_opr)?;
                let current_symbol_variant = CurrentSynSymbolVariant::FrameVariable {
                    expr_idx: particulars.for_between_loop_var_expr_idx,
                    ident: particulars.for_between_loop_var_ident,
                };
                let current_symbol_kind = current_symbol_variant.kind();
                let access_start = self.ast_token_idx_range_sheet[body.ast_idx_range().start()]
                    .start()
                    .token_idx();
                let access_end =
                    self.ast_token_idx_range_sheet[body.ast_idx_range().end() - 1].end();
                let frame_var_symbol = CurrentSynSymbol::new(
                    &self.pattern_expr_region,
                    access_start,
                    Some(access_end),
                    current_symbol_variant,
                );
                let frame_var_symbol_idx = self
                    .define_symbols(
                        vec![frame_var_symbol],
                        Some(ObeliskTypeConstraint::FrameVariable),
                    )
                    .start();
                self.expr_arena.set(
                    particulars.for_between_loop_var_expr_idx,
                    SynExpr::FrameVarDecl {
                        token_idx: particulars.for_between_loop_var_token_idx,
                        ident: particulars.for_between_loop_var_ident,
                        frame_var_symbol_idx,
                        current_symbol_kind,
                    },
                );
                Ok(SynStmt::ForBetween {
                    for_token,
                    particulars,
                    frame_var_symbol_idx,
                    eol_colon,
                    block: self.parse_block_stmts_expected(body, token_group_idx),
                })
            }
            SynExpr::Binary {
                lopd,
                opr: BinaryOpr::In,
                opr_token_idx,
                ropd,
            } => Ok(SynStmt::ForIn {
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
        lopd: SynExprIdx,
        ropd: SynExprIdx,
        comparison_opr: BinaryComparisonOpr,
    ) -> Result<SynForBetweenParticulars, StmtError> {
        use OriginalExprError::UnrecognizedIdent;
        let lopd_expr = &self.expr_arena[lopd];
        let ropd_expr = &self.expr_arena[ropd];
        // todo: parse with
        if let SynExpr::Err(ExprError::Original(UnrecognizedIdent { token_idx, ident })) = lopd_expr
        {
            Ok(SynForBetweenParticulars {
                for_between_loop_var_token_idx: *token_idx,
                for_between_loop_var_expr_idx: lopd,
                for_between_loop_var_ident: *ident,
                range: SynForBetweenRange::new_with_default_initial(comparison_opr, ropd)?,
            })
        } else if let SynExpr::Err(ExprError::Original(UnrecognizedIdent { token_idx, ident })) =
            ropd_expr
        {
            Ok(SynForBetweenParticulars {
                for_between_loop_var_token_idx: *token_idx,
                for_between_loop_var_expr_idx: ropd,
                for_between_loop_var_ident: *ident,
                range: SynForBetweenRange::new_with_default_final(lopd, comparison_opr)?,
            })
        } else {
            let final_comparison = comparison_opr;
            match lopd_expr {
                SynExpr::Binary {
                    lopd: llopd,
                    opr: BinaryOpr::Comparison(initial_comparison),
                    opr_token_idx,
                    ropd: lropd,
                } => {
                    let lropd_expr = &self.expr_arena[lropd];
                    match lropd_expr {
                        SynExpr::Err(ExprError::Original(UnrecognizedIdent {
                            token_idx,
                            ident,
                        })) => Ok(SynForBetweenParticulars {
                            for_between_loop_var_token_idx: *token_idx,
                            for_between_loop_var_expr_idx: *lropd,
                            for_between_loop_var_ident: *ident,
                            range: SynForBetweenRange::new_without_defaults(
                                *llopd,
                                *initial_comparison,
                                final_comparison,
                                ropd,
                            )?,
                        }),
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
    }

    fn parse_forext_loop_stmt(
        &mut self,
        token_group_idx: TokenGroupIdx,
        forext_token: ForextToken,
        expr: SynExprIdx,
        eol_colon: SynExprResult<EolToken>,
        body: FugitiveBody,
    ) -> StmtResult<SynStmt> {
        let SynExpr::Binary {
            lopd: forext_loop_var_expr_idx,
            opr: BinaryOpr::Comparison(opr),
            opr_token_idx,
            ropd: bound_expr,
        } = self.expr_arena[expr]
        else {
            todo!()
        };
        let (forext_loop_var_ident, forext_loop_var_token_idx) =
            match self.expr_arena[forext_loop_var_expr_idx] {
                SynExpr::InheritedSymbol {
                    ident,
                    token_idx,
                    inherited_symbol_idx,
                    inherited_symbol_kind,
                } => (ident, token_idx),
                SynExpr::CurrentSymbol {
                    ident,
                    token_idx,
                    current_symbol_idx,
                    current_symbol_kind,
                } => (ident, token_idx),
                _ => todo!(),
            };
        let particulars = SynForextParticulars::new(
            forext_loop_var_token_idx,
            forext_loop_var_ident,
            forext_loop_var_expr_idx,
            opr,
            bound_expr,
        );
        Ok(SynStmt::ForExt {
            forext_token,
            particulars,
            eol_colon,
            block: self.parse_block_stmts_expected(body, token_group_idx),
        })
    }

    fn parse_if_branch(&mut self, if_branch: AstIdx) -> SynIfBranch {
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
                SynIfBranch {
                    if_token: ctx.try_parse_option().unwrap().unwrap(),
                    condition: ctx.parse_expr_expected(
                        Some(ExprEnvironment::Condition(body_end)),
                        OriginalExprError::ExpectedCondition,
                    ),
                    eol_colon: ctx.try_parse_expected(OriginalExprError::ExpectedEolColon),
                    stmts: self.parse_block_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                }
            }
            _ => unreachable!(),
        }
    }
    fn form_body_end(&self, body: FugitiveBody) -> TokenIdxRangeEnd {
        self.ast_token_idx_range_sheet[body.ast_idx_range().end() - 1].end()
    }

    fn parse_elif_branches(&mut self, elif_branches: AstIdxRange) -> Vec<SynElifBranch> {
        elif_branches
            .into_iter()
            .map(|elif_branch| self.parse_elif_branch(elif_branch))
            .collect()
    }

    fn parse_elif_branch(&mut self, elif_branch: AstIdx) -> SynElifBranch {
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
                SynElifBranch {
                    elif_token: ctx.try_parse_option().unwrap().unwrap(),
                    condition: ctx.parse_expr_expected(
                        Some(ExprEnvironment::Condition(body_end)),
                        OriginalExprError::ExpectedCondition,
                    ),
                    eol_colon: ctx.try_parse_expected(OriginalExprError::ExpectedEolColon),
                    stmts: self.parse_block_stmts_expected(body, token_group_idx),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_else_branch(&mut self, else_branch: Option<AstIdx>) -> Option<SynElseBranch> {
        match self.ast_sheet[else_branch?] {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(token_group_idx, None);
                let mut ctx = self.ctx(token_stream);
                Some(SynElseBranch {
                    else_token: ctx.try_parse_option().unwrap().unwrap(),
                    eol_colon: ctx.try_parse_expected(OriginalExprError::ExpectedEolColon),
                    stmts: self.parse_block_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                })
            }
            _ => unreachable!(),
        }
    }

    pub fn finish(self) -> SynExprRegion {
        self.expr_parser.finish()
    }
}
