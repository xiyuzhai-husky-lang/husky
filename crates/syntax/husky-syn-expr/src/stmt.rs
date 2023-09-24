mod if_else_stmt;
mod loop_stmt;
mod match_stmt;

pub use self::if_else_stmt::*;
pub use self::loop_stmt::*;
pub use self::match_stmt::*;

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
        let_variables_pattern: SynExprResult<LetPatternObelisk>,
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
        block: SynStmtIdxRange,
    },
    ForIn {
        for_token: StmtForRegionalToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynStmtIdxRange,
    },
    ForExt {
        forext_token: ForextRegionalToken,
        particulars: SynForextParticulars,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynStmtIdxRange,
    },
    While {
        while_token: WhileRegionalToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynStmtIdxRange,
    },
    DoWhile {
        do_token: DoRegionalToken,
        while_token: WhileRegionalToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynStmtIdxRange,
    },
    IfElse {
        if_branch: SynIfBranch,
        elif_branches: Vec<SynElifBranch>,
        else_branch: Option<SynElseBranch>,
    },
    Match {
        match_token: MatchRegionalToken,
        match_expr: SynExprResult<SynExprIdx>,
        eol_with_token: SynExprResult<EolWithRegionalToken>,
        case_branches: Vec<SynCaseBranch>,
    },
}

impl<'a> SynStmtContext<'a> {
    pub(crate) fn parse_stmts(&mut self, body: DefnAstIdxRange) -> SynStmtIdxRange {
        let block_end = self.fugitive_body_end(body);
        let stmts = body
            .into_iter()
            .map(|ast_idx| self.parse_stmt(ast_idx, block_end))
            .collect();
        self.alloc_stmts(stmts)
    }

    pub fn parse_block_expr(&mut self, body: DefnAstIdxRange) -> SynExprIdx {
        let stmts = self.parse_stmts(body);
        let expr = self.alloc_expr(SynExpr::Block { stmts });
        self.add_expr_root(ExprRootKind::BlockExpr, expr);
        expr
    }

    fn parse_stmt(&mut self, ast_idx: DefnAstIdx, block_end: RegionalTokenIdxRangeEnd) -> SynStmt {
        match self.defn_tokra_region_data()[ast_idx] {
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
            DefnAst::MatchStmt {
                regional_token_group_idx: token_group_idx,
                pattern_stmt,
                case_branches,
                ..
            } => {
                let mut parser = self.expr_parser(token_group_idx);
                SynStmt::Match {
                    match_token: parser.try_parse_option().unwrap().unwrap(),
                    match_expr: parser.parse_expr_expected(
                        Some(ExprEnvironment::Condition(block_end)),
                        OriginalSynExprError::ExpectedMatchExpr,
                    ),
                    eol_with_token: parser
                        .try_parse_expected(OriginalSynExprError::ExpectedEolWithInMatchHead),
                    case_branches: self.parse_case_branches(case_branches),
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
                    block: self.parse_stmts(body.expect("should be checked in `husky_ast`")),
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
                            block: self
                                .parse_stmts(body.expect("should be checked in `husky_ast`")),
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
}
