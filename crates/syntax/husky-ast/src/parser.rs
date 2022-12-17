mod defn;
mod uses;

use crate::*;
use husky_token::{Keyword, SpecialToken, StmtKeyword, TokenGroupIter, TokenKind, TokenSheet};

pub(crate) struct AstParser<'a> {
    db: &'a dyn WordDb,
    arena: AstArena,
    token_sheet: &'a TokenSheet,
    token_groups: TokenGroupIter<'a>,
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(db: &'a dyn WordDb, token_sheet: &'a TokenSheet) -> Self {
        Self {
            db,
            arena: Default::default(),
            token_sheet,
            token_groups: token_sheet.iter(),
        }
    }

    pub(crate) fn parse_all(mut self) -> AstSheet {
        let top_level_asts = self.parse_asts(0);
        AstSheet::new(self.arena, top_level_asts)
    }

    fn parse_asts(&mut self, indent: u32) -> Option<AstIdxRange> {
        let mut asts: Vec<Ast> = vec![];
        let mut token_group_indices: Vec<TokenGroupIdx> = vec![];
        while let Some(ast) = self.parse_ast(indent) {
            asts.push(ast)
        }
        self.alloc_asts(asts)
    }

    fn alloc_asts(&mut self, asts: Vec<Ast>) -> Option<AstIdxRange> {
        self.arena.alloc_batch(asts)
    }

    fn alloc_ast(&mut self, ast: Ast) -> AstIdx {
        self.arena.alloc_one(ast)
    }

    fn parse_ast(&mut self, indent: u32) -> Option<Ast> {
        let (idx, token_group) = self.token_groups.next_with_equal_or_more_indent(indent)?;
        if token_group.indent() > indent {
            return Some(Ast::Err(idx, AstError::ExcessiveIndent));
        }
        Some(match token_group.first().kind {
            TokenKind::Decorator(_) => self.parse_defn_or_use(idx, indent),
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
                    | StmtKeyword::Require => self.parse_stmt(idx, indent),
                },
                Keyword::Liason(_) => todo!(),
                Keyword::Use => Ast::Use { token_group: idx },
                Keyword::Main => Ast::Main {
                    token_group: idx,
                    body: self.parse_asts(indent + INDENT_INCR),
                },
                Keyword::Config(_) => Ast::Config {
                    token_group: idx,
                    body: self.parse_asts(indent + INDENT_INCR),
                },
                Keyword::Mod | Keyword::Paradigm(_) | Keyword::Visual | Keyword::Type(_) => {
                    self.parse_defn(idx, indent)
                }
                Keyword::Impl => Ast::Impl {
                    token_group: idx,
                    body: self.parse_asts(indent + INDENT_INCR),
                },
                Keyword::End(_) => unreachable!(),
            },
            TokenKind::Special(SpecialToken::PoundSign) => Ast::Decor(idx),
            _ => self.parse_stmt(idx, indent),
        })
    }

    fn parse_if_else_stmts(&mut self, idx: TokenGroupIdx, indent: u32) -> Ast {
        Ast::IfElseStmts {
            if_stmt: self.alloc_stmt(idx, indent),
            elif_stmts: self.alloc_elif_stmts(indent),
            else_stmt: self.alloc_else_stmt(indent),
        }
    }

    fn alloc_elif_stmts(&mut self, indent: u32) -> Option<AstIdxRange> {
        let mut elif_stmts = vec![];
        while let Some((idx, token_group)) = self.token_groups.peek_with_exact_indent(indent) {
            match token_group.first().kind {
                TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    self.token_groups.next();
                    elif_stmts.push(self.parse_stmt(idx, indent))
                }
                _ => break,
            }
        }
        self.alloc_asts(elif_stmts)
    }

    fn alloc_else_stmt(&mut self, indent: u32) -> Option<AstIdx> {
        let (idx, token_group) = self.token_groups.peek_with_exact_indent(indent)?;
        match token_group.first().kind {
            TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                self.token_groups.next();
                Some(self.alloc_stmt(idx, indent))
            }
            _ => None,
        }
    }

    fn parse_match_stmts(&mut self, token_group: TokenGroupIdx, indent: u32) -> Ast {
        Ast::MatchStmts {
            pattern_stmt: self.alloc_stmt(token_group, indent),
            case_stmts: self.parse_case_stmts(indent),
        }
    }

    fn alloc_stmt(&mut self, token_group: TokenGroupIdx, indent: u32) -> AstIdx {
        let ast = self.parse_stmt(token_group, indent);
        self.alloc_ast(ast)
    }

    fn parse_stmt(&mut self, token_group: TokenGroupIdx, indent: u32) -> Ast {
        let body = self.parse_asts(indent + INDENT_INCR);
        Ast::Stmt { token_group, body }
    }

    fn parse_case_stmts(&mut self, indent: u32) -> Option<AstIdxRange> {
        let mut case_stmts = vec![];
        while let Some((idx, token_group)) = self.token_groups.peek_with_exact_indent(indent) {
            match token_group.first().kind {
                TokenKind::Special(SpecialToken::Vertical) => {
                    self.token_groups.next();
                    case_stmts.push(self.parse_stmt(idx, indent))
                }
                _ => break,
            }
        }
        self.alloc_asts(case_stmts)
    }

    fn parse_defn_or_use(&mut self, token_group: TokenGroupIdx, indent: u32) -> Ast {
        for token in &self.token_sheet[token_group] {
            match token.kind {
                TokenKind::Decorator(_) | TokenKind::Comment => (),
                TokenKind::Keyword(Keyword::Use) => return self.parse_uses(token_group, indent),
                TokenKind::Keyword(_) => return self.parse_defn(token_group, indent),
                TokenKind::Identifier(_)
                | TokenKind::Special(_)
                | TokenKind::WordOpr(_)
                | TokenKind::Literal(_)
                | TokenKind::Unrecognized(_)
                | TokenKind::IllFormedLiteral(_) => {
                    return Ast::Err(token_group, AstError::ExpectDecoratorOrEntityKeyword)
                }
            }
        }
        Ast::Err(token_group, todo!())
    }
}
