use super::*;

impl<'a> AstParser<'a> {
    #[inline(always)]
    pub(super) fn try_parse_stmt<C: IsAstChildren>(
        &mut self,
        token_verse_idx: TokenVerseIdx,
    ) -> AstResult<AstData> {
        C::ALLOW_STMT?;
        Ok(self.parse_stmt(token_verse_idx))
    }

    #[inline(always)]
    pub(super) fn try_parse_stmt_after_keyword<C: IsAstChildren>(
        &mut self,
        token_verse_idx: TokenVerseIdx,
        keyword: StmtKeyword,
    ) -> AstResult<AstData> {
        C::ALLOW_STMT?;
        Ok(match keyword {
            StmtKeyword::If => self.parse_if_else_stmts(token_verse_idx),
            StmtKeyword::Elif => Err(OriginalAstError::StandaloneElif)?,
            StmtKeyword::Else => Err(OriginalAstError::StandaloneElse)?,
            StmtKeyword::Match => self.parse_match_stmts(token_verse_idx),
            StmtKeyword::While
            | StmtKeyword::Do
            | StmtKeyword::NonImplFor
            | StmtKeyword::Forext
            | StmtKeyword::Let
            | StmtKeyword::Break
            | StmtKeyword::Return
            | StmtKeyword::Assert
            | StmtKeyword::Require => self.parse_stmt(token_verse_idx),
        })
    }

    fn parse_if_else_stmts(&mut self, idx: TokenVerseIdx) -> AstData {
        AstData::IfElseStmts {
            if_branch: self.alloc_stmt(idx),
            elif_branches: self.alloc_elif_stmts(),
            else_branch: self.alloc_else_stmt(),
        }
    }

    fn alloc_elif_stmts(&mut self) -> AstIdxRange {
        let mut elif_stmts = vec![];
        while let Some((idx, _token_verse, first_noncomment_token)) = self
            .token_verse_iter
            .peek_token_verse_of_exact_indent_with_its_first_token(self.indent())
        {
            match first_noncomment_token {
                TokenData::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    self.token_verse_iter.next();
                    elif_stmts.push(self.parse_stmt(idx))
                }
                _ => break,
            }
        }
        self.alloc_asts(elif_stmts)
    }

    fn alloc_else_stmt(&mut self) -> Option<AstIdx> {
        let (idx, _token_verse, first_noncomment_token) = self
            .token_verse_iter
            .peek_token_verse_of_exact_indent_with_its_first_token(self.indent())?;
        match first_noncomment_token {
            TokenData::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                self.token_verse_iter.next();
                Some(self.alloc_stmt(idx))
            }
            _ => None,
        }
    }

    fn parse_match_stmts(&mut self, token_verse_idx: TokenVerseIdx) -> AstData {
        AstData::MatchStmt {
            token_verse_idx,
            pattern_stmt: self.alloc_stmt(token_verse_idx),
            case_branches: self.parse_case_stmts(),
        }
    }

    fn alloc_stmt(&mut self, token_verse_idx: TokenVerseIdx) -> AstIdx {
        let ast = self.parse_stmt(token_verse_idx);
        self.alloc_ast(ast)
    }

    fn parse_stmt(&mut self, token_verse_idx: TokenVerseIdx) -> AstData {
        let body = match self.try_parse_option() {
            Ok(body) => body,
            Err(error) => {
                return AstData::Err {
                    token_verse_idx,
                    error,
                }
            }
        };
        AstData::BasicStmtOrBranch {
            token_verse_idx,
            body,
        }
    }

    fn parse_case_stmts(&mut self) -> AstIdxRange {
        let mut verticals = vec![];
        while let Some((idx, _token_verse, first)) = self
            .token_verse_iter
            .peek_token_verse_of_exact_indent_with_its_first_token(self.indent())
        {
            match first {
                TokenData::Punctuation(Punctuation::VERT) => {
                    self.token_verse_iter.next();
                    verticals.push(self.parse_stmt(idx))
                }
                _ => break,
            }
        }
        self.alloc_asts(verticals)
    }
}
