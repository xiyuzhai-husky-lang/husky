use crate::*;
use husky_token::{Keyword, SpecialToken, StmtKeyword, TokenGroupIter, TokenGroupSheet, TokenKind};

pub(crate) struct AstParser<'a> {
    db: &'a dyn WordDb,
    arena: AstArena,
    token_groups: TokenGroupIter<'a>,
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(db: &'a dyn WordDb, token_sheet: &'a TokenGroupSheet) -> Self {
        Self {
            db,
            arena: Default::default(),
            token_groups: token_sheet.iter(),
        }
    }

    pub(crate) fn parse_all(mut self) -> AstSheet {
        let top_level_asts = self.parse_asts(0);
        AstSheet::new(self.arena, top_level_asts)
    }

    fn parse_asts(&mut self, indent: u32) -> AstIdxRange {
        let mut asts: Vec<Ast> = vec![];
        let mut token_group_indices: Vec<TokenGroupIdx> = vec![];
        while let Some(ast) = self.parse_ast(indent) {
            asts.push(ast)
        }
        self.alloc_asts(asts)
    }

    fn alloc_asts(&mut self, asts: Vec<Ast>) -> AstIdxRange {
        self.arena.alloc_batch(asts)
    }

    fn parse_ast(&mut self, indent: u32) -> Option<Ast> {
        let (idx, token_group) = self.token_groups.next_with_equal_or_more_indent(indent)?;
        if token_group.indent() > indent {
            return Some(Ast::Err(idx, AstError::ExcessiveIndent));
        }
        Some(match token_group.first().kind {
            TokenKind::Decorator(_) => Ast::Defn(idx, self.parse_asts(indent + INDENT_INCR)),
            TokenKind::Keyword(kw) => match kw {
                Keyword::Stmt(kw) => match kw {
                    StmtKeyword::If => self.parse_if_else_stmts(idx, indent),
                    StmtKeyword::Elif => Ast::Err(idx, AstError::StandaloneElif),
                    StmtKeyword::Else => Ast::Err(idx, AstError::StandaloneElse),
                    StmtKeyword::Match => self.parse_match_stmts(idx, indent),
                    StmtKeyword::While
                    | StmtKeyword::Do
                    | StmtKeyword::For
                    | StmtKeyword::ForExt
                    | StmtKeyword::Let
                    | StmtKeyword::Var
                    | StmtKeyword::Break
                    | StmtKeyword::Return
                    | StmtKeyword::Assert
                    | StmtKeyword::Require => Ast::Stmt(idx, self.parse_asts(indent + INDENT_INCR)),
                },
                Keyword::Liason(_) => todo!(),
                Keyword::Use => Ast::Use(idx),
                Keyword::Mod => Ast::Mod(idx),
                Keyword::Main
                | Keyword::Config(_)
                | Keyword::Paradigm(_)
                | Keyword::Visual
                | Keyword::Type(_)
                | Keyword::Impl => Ast::Defn(idx, self.parse_asts(indent + INDENT_INCR)),
                Keyword::End(_) => todo!(),
            },
            TokenKind::Special(SpecialToken::PoundSign) => Ast::Decor(idx),
            _ => Ast::Stmt(idx, self.parse_asts(indent + INDENT_INCR)),
        })
    }

    fn parse_if_else_stmts(&mut self, idx: TokenGroupIdx, indent: u32) -> Ast {
        Ast::IfElseStmts {
            if_stmt: (idx, self.parse_asts(indent + INDENT_INCR)),
            elif_stmts: self.parse_elif_stmts(indent),
            else_stmt: self.parse_else_stmt(indent),
        }
    }

    fn parse_elif_stmts(&mut self, indent: u32) -> Vec<(TokenGroupIdx, AstIdxRange)> {
        let mut elif_stmts = vec![];
        while let Some((idx, token_group)) = self.token_groups.peek_with_exact_indent(indent) {
            match token_group.first().kind {
                TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    self.token_groups.next();
                    elif_stmts.push((idx, self.parse_asts(indent + INDENT_INCR)))
                }
                _ => break,
            }
        }
        elif_stmts
    }

    fn parse_else_stmt(&mut self, indent: u32) -> Option<(TokenGroupIdx, AstIdxRange)> {
        let (idx, token_group) = self.token_groups.peek_with_exact_indent(indent)?;
        match token_group.first().kind {
            TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                self.token_groups.next();
                Some((idx, self.parse_asts(indent + INDENT_INCR)))
            }
            _ => None,
        }
    }

    fn parse_match_stmts(&mut self, idx: TokenGroupIdx, indent: u32) -> Ast {
        Ast::MatchStmts {
            pattern_stmt: (idx, self.parse_asts(indent + INDENT_INCR)),
            case_stmts: self.parse_case_stmts(indent),
        }
    }

    fn parse_case_stmts(&mut self, indent: u32) -> Vec<(TokenGroupIdx, AstIdxRange)> {
        let mut case_stmts = vec![];
        while let Some((idx, token_group)) = self.token_groups.peek_with_exact_indent(indent) {
            match token_group.first().kind {
                TokenKind::Special(SpecialToken::Vertical) => {
                    self.token_groups.next();
                    case_stmts.push((idx, self.parse_asts(indent + INDENT_INCR)))
                }
                _ => break,
            }
        }
        case_stmts
    }
}
