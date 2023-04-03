use super::*;

impl<'a> AstParser<'a> {
    #[inline(always)]
    pub(super) fn try_parse_stmt<C: NormalAstChildren>(
        &mut self,
        token_group_idx: TokenGroupIdx,
    ) -> AstResult<Ast> {
        C::ALLOW_STMT?;
        Ok(self.parse_stmt(token_group_idx))
    }

    #[inline(always)]
    pub(super) fn try_parse_stmt_after_keyword<C: NormalAstChildren>(
        &mut self,
        token_group_idx: TokenGroupIdx,
        keyword: StmtKeyword,
    ) -> AstResult<Ast> {
        C::ALLOW_STMT?;
        Ok(match keyword {
            StmtKeyword::If => self.parse_if_else_stmts(token_group_idx),
            StmtKeyword::Elif => Err(OriginalAstError::StandaloneElif)?,
            StmtKeyword::Else => Err(OriginalAstError::StandaloneElse)?,
            StmtKeyword::Match => self.parse_match_stmts(token_group_idx),
            StmtKeyword::While
            | StmtKeyword::Do
            | StmtKeyword::NonImplFor
            | StmtKeyword::ForExt
            | StmtKeyword::Let
            | StmtKeyword::Break
            | StmtKeyword::Return
            | StmtKeyword::Assert
            | StmtKeyword::Require => self.parse_stmt(token_group_idx),
        })
    }

    fn parse_if_else_stmts(&mut self, idx: TokenGroupIdx) -> Ast {
        Ast::IfElseStmts {
            if_branch: self.alloc_stmt(idx),
            elif_branches: self.alloc_elif_stmts(),
            else_branch: self.alloc_else_stmt(),
        }
    }

    fn alloc_elif_stmts(&mut self) -> AstIdxRange {
        let mut elif_stmts = vec![];
        while let Some((idx, token_group, first_noncomment_token)) = self
            .token_groups
            .peek_token_group_of_exact_indent_with_its_first_token(self.indent())
        {
            match first_noncomment_token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    self.token_groups.next();
                    elif_stmts.push(self.parse_stmt(idx))
                }
                _ => break,
            }
        }
        self.alloc_asts(elif_stmts)
    }

    fn alloc_else_stmt(&mut self) -> Option<AstIdx> {
        let (idx, token_group, first_noncomment_token) = self
            .token_groups
            .peek_token_group_of_exact_indent_with_its_first_token(self.indent())?;
        match first_noncomment_token {
            Token::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                self.token_groups.next();
                Some(self.alloc_stmt(idx))
            }
            _ => None,
        }
    }

    fn parse_match_stmts(&mut self, token_group_idx: TokenGroupIdx) -> Ast {
        Ast::MatchStmts {
            token_group_idx,
            pattern_stmt: self.alloc_stmt(token_group_idx),
            case_stmts: self.parse_case_stmts(),
        }
    }

    fn alloc_stmt(&mut self, token_group_idx: TokenGroupIdx) -> AstIdx {
        let ast = self.parse_stmt(token_group_idx);
        self.alloc_ast(ast)
    }

    fn parse_stmt(&mut self, token_group_idx: TokenGroupIdx) -> Ast {
        let body = match self.parse() {
            Ok(body) => body,
            Err(_) => todo!(),
        };
        Ast::BasicStmtOrBranch {
            token_group_idx,
            body,
        }
    }

    fn parse_case_stmts(&mut self) -> AstIdxRange {
        let mut verticals = vec![];
        while let Some((idx, token_group, first)) = self
            .token_groups
            .peek_token_group_of_exact_indent_with_its_first_token(self.indent())
        {
            match first {
                Token::Punctuation(Punctuation::VERTICAL) => {
                    self.token_groups.next();
                    verticals.push(self.parse_stmt(idx))
                }
                _ => break,
            }
        }
        self.alloc_asts(verticals)
    }
}
