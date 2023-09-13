use super::*;
use husky_defn_ast::DefnAst;

#[derive(Debug, PartialEq, Eq)]
pub struct SynIfBranch {
    pub if_token: IfRegionalToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolRegionalToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl SynIfBranch {
    pub fn condition(&self) -> Result<SynExprIdx, &SynExprError> {
        self.condition.as_ref().copied()
    }

    pub fn eol_colon_token(&self) -> Result<EolRegionalToken, &SynExprError> {
        self.eol_colon.as_ref().copied()
    }

    pub fn stmts(&self) -> Result<SynStmtIdxRange, &SynExprError> {
        self.stmts.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynElifBranch {
    pub elif_token: ElifRegionalToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolRegionalToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl SynElifBranch {
    pub fn condition(&self) -> Result<SynExprIdx, &SynExprError> {
        self.condition.as_ref().copied()
    }

    pub fn eol_colon(&self) -> Result<EolRegionalToken, &SynExprError> {
        self.eol_colon.as_ref().copied()
    }

    pub fn stmts(&self) -> Result<SynStmtIdxRange, &SynExprError> {
        self.stmts.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynElseBranch {
    pub else_token: ElseRegionalToken,
    pub eol_colon: SynExprResult<EolRegionalToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl SynElseBranch {
    pub fn stmts(&self) -> Result<SynStmtIdxRange, &SynExprError> {
        self.stmts.as_ref().copied()
    }
}

impl<'a> SynStmtContext<'a> {
    pub(super) fn parse_if_branch(&mut self, if_branch: DefnAstIdx) -> SynIfBranch {
        match self.regional_asts()[if_branch] {
            DefnAst::BasicStmtOrBranch {
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

    pub(super) fn parse_elif_branches(
        &mut self,
        elif_branches: DefnAstIdxRange,
    ) -> Vec<SynElifBranch> {
        elif_branches
            .into_iter()
            .map(|elif_branch| self.parse_elif_branch(elif_branch))
            .collect()
    }

    fn parse_elif_branch(&mut self, elif_branch: DefnAstIdx) -> SynElifBranch {
        match self.regional_asts()[elif_branch] {
            DefnAst::BasicStmtOrBranch {
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
        else_branch: Option<DefnAstIdx>,
    ) -> Option<SynElseBranch> {
        match self.regional_asts()[else_branch?] {
            DefnAst::BasicStmtOrBranch {
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
