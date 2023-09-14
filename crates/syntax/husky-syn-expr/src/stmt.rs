mod branch_stmt;
mod loop_stmt;

pub use self::branch_stmt::*;
pub use self::loop_stmt::*;

use crate::*;
use husky_defn_ast::{DefnAst, DefnAstIdx, DefnAstIdxRange};
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use parsec::StreamParser;

pub type SynStmtArena = Arena<SynStmt>;
pub type SynStmtIdx = ArenaIdx<SynStmt>;
pub type SynStmtIdxRange = ArenaIdxRange<SynStmt>;
pub type SynStmtMap<V> = ArenaMap<SynStmt, V>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynStmt {
    Let {
        let_token: LetRegionalToken,
        let_variables_pattern: SynExprResult<LetVariableObelisk>,
        assign_token: SynExprResult<RegionalEqToken>,
        initial_value: SynExprIdx,
    },
    Return {
        return_token: ReturnRegionalToken,
        result: SynExprIdx,
    },
    Require {
        require_token: RequireRegionalToken,
        condition: SynExprIdx,
    },
    Assert {
        assert_token: AssertRegionalToken,
        condition: SynExprIdx,
    },
    Break {
        break_token: BreakRegionalToken,
    },
    Eval {
        expr_idx: SynExprIdx,
        // todo: change this to EolOrEolSemicolonToken
        eol_semicolon: TokenDataResult<Option<EolSemicolonRegionalToken>>,
    },
    ForBetween {
        for_token: StmtForRegionalToken,
        particulars: SynForBetweenParticulars,
        frame_var_symbol_idx: CurrentSynSymbolIdx,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    ForIn {
        for_token: StmtForRegionalToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    ForExt {
        forext_token: ForextRegionalToken,
        particulars: SynForextParticulars,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    While {
        while_token: WhileRegionalToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    DoWhile {
        do_token: DoRegionalToken,
        while_token: WhileRegionalToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    IfElse {
        if_branch: SynIfBranch,
        elif_branches: Vec<SynElifBranch>,
        else_branch: Option<SynElseBranch>,
    },
    Match {
        match_token: MatchRegionalToken,
    },
}

impl<'a> SynStmtContext<'a> {
    pub fn parse_stmts_expected(
        &mut self,
        body: DefnAstIdxRange,
        token_group_idx: RegionalTokenGroupIdx,
    ) -> SynExprResult<SynStmtIdxRange> {
        match self.parse_stmts(body) {
            Some(stmt_idx_range) => Ok(stmt_idx_range),
            None => Err(OriginalSynExprError::ExpectedBlock(token_group_idx).into()),
        }
    }

    pub(crate) fn parse_stmts(&mut self, body: DefnAstIdxRange) -> Option<SynStmtIdxRange> {
        let block_end = self.fugitive_body_end(body);
        if body.len() == 0 {
            return None;
        }
        let stmts = body
            .into_iter()
            .map(|ast_idx| self.parse_stmt(ast_idx, block_end))
            .collect();
        Some(self.alloc_stmts(stmts))
    }

    pub fn parse_block_expr(&mut self, body: DefnAstIdxRange) -> SynExprIdx {
        let stmts = self
            .parse_stmts(body)
            .expect("husky-ast should guarantee that this not empty");
        let expr = self.alloc_expr(SynExpr::Block { stmts });
        self.add_expr_root(ExprRootKind::BlockExpr, expr);
        expr
    }

    fn parse_stmt(&mut self, ast_idx: DefnAstIdx, block_end: RegionalTokenIdxRangeEnd) -> SynStmt {
        let ast = todo!();
        match ast {
            DefnAst::BasicStmtOrBranch {
                regional_token_group_idx: token_group_idx,
                body,
            } => self.parse_basic_stmt(token_group_idx, block_end, body),
            DefnAst::IfElseStmts {
                if_branch,
                elif_branches,
                else_branch,
            } => SynStmt::IfElse {
                if_branch: self.parse_if_branch(if_branch),
                elif_branches: self.parse_elif_branches(elif_branches),
                else_branch: self.parse_else_branch(else_branch),
            },
            DefnAst::MatchStmts {
                token_group_idx,
                pattern_stmt,
                case_stmts,
                ..
            } => {
                let mut token_stream = self.token_group_token_stream(token_group_idx, None);
                SynStmt::Match {
                    match_token: token_stream.try_parse_option().unwrap().unwrap(),
                }
            }
            DefnAst::Err { .. } => todo!(),
        }
    }

    fn parse_basic_stmt(
        &mut self,
        token_group_idx: RegionalTokenGroupIdx,
        block_end: RegionalTokenIdxRangeEnd,
        body: Option<DefnAstIdxRange>,
    ) -> SynStmt {
        let mut parser = self.expr_parser(token_group_idx);
        match parser.try_parse_option::<BasicStmtKeywordRegionalToken>() {
            Ok(Some(basic_stmt_keyword_token)) => match basic_stmt_keyword_token {
                BasicStmtKeywordRegionalToken::Let(let_token) => SynStmt::Let {
                    let_token,
                    let_variables_pattern: parser.parse_let_variables_pattern_expected(block_end),
                    assign_token: parser.try_parse_expected(OriginalSynExprError::ExpectedAssign),
                    initial_value: parser.parse_expr_expected2(
                        None,
                        ExprRootKind::LetStmtInitialValue,
                        OriginalSynExprError::ExpectedInitialValue,
                    ),
                },
                BasicStmtKeywordRegionalToken::Return(return_token) => SynStmt::Return {
                    return_token,
                    result: parser.parse_expr_expected2(
                        None,
                        ExprRootKind::ReturnExpr,
                        OriginalSynExprError::ExpectedResult,
                    ),
                },
                BasicStmtKeywordRegionalToken::Require(require_token) => SynStmt::Require {
                    require_token,
                    condition: parser.parse_expr_expected2(
                        Some(ExprEnvironment::Condition(block_end)),
                        ExprRootKind::Condition,
                        OriginalSynExprError::ExpectedCondition,
                    ),
                },
                BasicStmtKeywordRegionalToken::Assert(assert_token) => SynStmt::Assert {
                    assert_token,
                    condition: parser.parse_expr_expected2(
                        Some(ExprEnvironment::Condition(block_end)),
                        ExprRootKind::Condition,
                        OriginalSynExprError::ExpectedCondition,
                    ),
                },
                BasicStmtKeywordRegionalToken::Break(break_token) => SynStmt::Break { break_token },
                BasicStmtKeywordRegionalToken::For(for_token) => {
                    let expr = match parser
                        .parse_expr_expected(None, OriginalSynExprError::ExpectedCondition)
                    {
                        Ok(expr) => expr,
                        Err(_) => todo!(),
                    };
                    let eol_colon =
                        parser.try_parse_expected(OriginalSynExprError::ExpectedEolColon);
                    self.parse_for_loop_stmt(
                        token_group_idx,
                        for_token,
                        expr,
                        eol_colon,
                        body.expect("should be checked in `husky_ast`"),
                    )
                    .into()
                }
                BasicStmtKeywordRegionalToken::ForExt(forext_token) => {
                    let expr = match parser
                        .parse_expr_expected(None, OriginalSynExprError::ExpectedCondition)
                    {
                        Ok(expr) => expr,
                        Err(_) => todo!(),
                    };
                    let eol_colon =
                        parser.try_parse_expected(OriginalSynExprError::ExpectedEolColon);
                    self.parse_forext_loop_stmt(
                        token_group_idx,
                        forext_token,
                        expr,
                        eol_colon,
                        body.expect("should be checked in `husky_ast`"),
                    )
                    .into()
                }
                BasicStmtKeywordRegionalToken::While(while_token) => SynStmt::While {
                    while_token,
                    condition: parser.parse_expr_expected(
                        Some(ExprEnvironment::Condition(block_end)),
                        OriginalSynExprError::ExpectedCondition,
                    ),
                    eol_colon: parser.try_parse_expected(OriginalSynExprError::ExpectedEolColon),
                    block: self.parse_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                },
                BasicStmtKeywordRegionalToken::Do(do_token) => {
                    match parser.try_parse_option::<WhileRegionalToken>() {
                        Ok(Some(while_token)) => SynStmt::DoWhile {
                            do_token,
                            while_token,
                            condition: parser.parse_expr_expected(
                                Some(ExprEnvironment::Condition(block_end)),
                                OriginalSynExprError::ExpectedCondition,
                            ),
                            eol_colon: parser
                                .try_parse_expected(OriginalSynExprError::ExpectedEolColon),
                            block: self.parse_stmts_expected(
                                body.expect("should be checked in `husky_ast`"),
                                token_group_idx,
                            ),
                        },
                        Ok(None) => todo!(),
                        Err(_) => todo!(),
                    }
                }
            },
            Ok(None) => match parser.parse_expr_root(None, ExprRootKind::EvalExpr) {
                Some(expr_idx) => SynStmt::Eval {
                    expr_idx,
                    eol_semicolon: parser.try_parse_option(),
                },
                None => todo!(),
            },
            Err(_) => todo!(),
        }
    }

    fn fugitive_body_end(&self, body: DefnAstIdxRange) -> RegionalTokenIdxRangeEnd {
        todo!()
        // self.ast_token_idx_range_sheet()[body.ast_idx_range().end() - 1].end()
    }
}
