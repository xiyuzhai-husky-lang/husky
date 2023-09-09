use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SynIfBranch {
    pub if_token: IfToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl SynIfBranch {
    pub fn condition(&self) -> Result<SynExprIdx, &SynExprError> {
        self.condition.as_ref().copied()
    }

    pub fn eol_colon_token(&self) -> Result<EolToken, &SynExprError> {
        self.eol_colon.as_ref().copied()
    }

    pub fn stmts(&self) -> Result<SynStmtIdxRange, &SynExprError> {
        self.stmts.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynElifBranch {
    pub elif_token: ElifToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl SynElifBranch {
    pub fn condition(&self) -> Result<SynExprIdx, &SynExprError> {
        self.condition.as_ref().copied()
    }

    pub fn eol_colon(&self) -> Result<EolToken, &SynExprError> {
        self.eol_colon.as_ref().copied()
    }

    pub fn stmts(&self) -> Result<SynStmtIdxRange, &SynExprError> {
        self.stmts.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynElseBranch {
    pub else_token: ElseToken,
    pub eol_colon: SynExprResult<EolToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl SynElseBranch {
    pub fn stmts(&self) -> Result<SynStmtIdxRange, &SynExprError> {
        self.stmts.as_ref().copied()
    }
}

impl<'a> StmtContext<'a> {
    pub(super) fn parse_if_branch(&mut self, if_branch: AstIdx) -> SynIfBranch {
        match self.ast_sheet()[if_branch] {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => {
                let body_end =
                    self.fugitive_body_end(body.expect("should be checked in `husky_ast`"));
                let mut parser = self.expr_parser(token_group_idx);
                SynIfBranch {
                    if_token: parser.try_parse_option().unwrap().unwrap(),
                    condition: parser.parse_expr_expected(
                        Some(ExprEnvironment::Condition(body_end)),
                        OriginalSynExprError::ExpectedCondition,
                    ),
                    eol_colon: parser.try_parse_expected(OriginalSynExprError::ExpectedEolColon),
                    stmts: self.parse_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                }
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn parse_elif_branches(&mut self, elif_branches: AstIdxRange) -> Vec<SynElifBranch> {
        elif_branches
            .into_iter()
            .map(|elif_branch| self.parse_elif_branch(elif_branch))
            .collect()
    }

    fn parse_elif_branch(&mut self, elif_branch: AstIdx) -> SynElifBranch {
        match self.ast_sheet()[elif_branch] {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => {
                let body = body.expect("should be checked in `husky_ast`");
                let body_end = self.fugitive_body_end(body);
                let mut parser = self.expr_parser(token_group_idx);
                SynElifBranch {
                    elif_token: parser.try_parse_option().unwrap().unwrap(),
                    condition: parser.parse_expr_expected(
                        Some(ExprEnvironment::Condition(body_end)),
                        OriginalSynExprError::ExpectedCondition,
                    ),
                    eol_colon: parser.try_parse_expected(OriginalSynExprError::ExpectedEolColon),
                    stmts: self.parse_stmts_expected(body, token_group_idx),
                }
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn parse_else_branch(
        &mut self,
        else_branch: Option<AstIdx>,
    ) -> Option<SynElseBranch> {
        match self.ast_sheet()[else_branch?] {
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => {
                let mut parser = self.expr_parser(token_group_idx);
                Some(SynElseBranch {
                    else_token: parser.try_parse_option().unwrap().unwrap(),
                    eol_colon: parser.try_parse_expected(OriginalSynExprError::ExpectedEolColon),
                    stmts: self.parse_stmts_expected(
                        body.expect("should be checked in `husky_ast`"),
                        token_group_idx,
                    ),
                })
            }
            _ => unreachable!(),
        }
    }
}
