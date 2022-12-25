mod context;
mod defn;
mod uses;
mod utils;

use crate::*;
use context::*;
use husky_token::{Keyword, SpecialToken, StmtKeyword, TokenGroupIter, TokenKind, TokenSheet};
use utils::*;

pub(crate) struct AstParser<'a> {
    db: &'a dyn AstDb,
    module_path: ModulePath,
    token_sheet: &'a TokenSheet,
    token_groups: TokenGroupIter<'a>,
    ast_arena: AstArena,
    use_expr_arena: UseExprArena,
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(db: &'a dyn AstDb, module_path: ModulePath) -> VfsResult<Self> {
        let token_sheet = &db.token_sheet(module_path)?;
        Ok(Self {
            db,
            module_path,
            token_sheet,
            token_groups: token_sheet.token_group_iter(),
            ast_arena: Default::default(),
            use_expr_arena: Default::default(),
        })
    }

    pub(crate) fn parse_all(mut self) -> AstSheet {
        let top_level_asts = self.parse_asts(Context::new_module());
        AstSheet::new(self.ast_arena, top_level_asts, self.use_expr_arena)
    }

    fn parse_asts(&mut self, context: Context) -> AstIdxRange {
        let mut asts: Vec<Ast> = vec![];
        let _token_group_indices: Vec<TokenGroupIdx> = vec![];
        while let Some(ast) = self.parse_ast(&context) {
            asts.push(ast)
        }
        self.alloc_asts(asts)
    }

    fn alloc_asts(&mut self, asts: Vec<Ast>) -> AstIdxRange {
        self.ast_arena.alloc_batch(asts)
    }

    fn alloc_ast(&mut self, ast: Ast) -> AstIdx {
        self.ast_arena.alloc_one(ast)
    }

    fn parse_ast(&mut self, context: &Context) -> Option<Ast> {
        let (token_group_idx, token_group) = self
            .token_groups
            .next_with_equal_or_more_indent(context.indent())?;
        if token_group.indent() > context.indent() {
            return Some(Ast::Err {
                token_group_idx,
                error: AstError::ExcessiveIndent,
            });
        }
        Some(match token_group.first().kind {
            TokenKind::Attr(_) => self.parse_defn_or_use(token_group_idx, context),
            TokenKind::Keyword(kw) => match kw {
                Keyword::Stmt(kw) => match kw {
                    StmtKeyword::If => self.parse_if_else_stmts(token_group_idx, &context),
                    StmtKeyword::Elif => Ast::Err {
                        token_group_idx,
                        error: AstError::StandaloneElif,
                    },
                    StmtKeyword::Else => Ast::Err {
                        token_group_idx,
                        error: AstError::StandaloneElse,
                    },
                    StmtKeyword::Match => self.parse_match_stmts(token_group_idx, &context),
                    StmtKeyword::While
                    | StmtKeyword::Do
                    | StmtKeyword::For
                    | StmtKeyword::ForExt
                    | StmtKeyword::Let
                    | StmtKeyword::Var
                    | StmtKeyword::Break
                    | StmtKeyword::Return
                    | StmtKeyword::Assert
                    | StmtKeyword::Require => self.parse_stmt(token_group_idx, &context),
                },
                Keyword::Liason(_) => todo!(),
                Keyword::Use => self.parse_uses(token_group_idx, context),
                Keyword::Main => Ast::Main {
                    token_group_idx,
                    body: self.parse_asts(context.subcontext(AstContextKind::InsideForm)),
                },
                Keyword::Config(_) => Ast::Config {
                    token_group_idx,
                    body: self.parse_asts(context.subcontext(AstContextKind::InsideForm)),
                },
                Keyword::Mod
                | Keyword::Paradigm(_)
                | Keyword::Visual
                | Keyword::Trait
                | Keyword::Type(_) => self.parse_defn(context, token_group_idx),
                Keyword::Impl => Ast::Impl {
                    token_group_idx,
                    body: self.parse_asts(context.subcontext(AstContextKind::InsideImpl)),
                },
                Keyword::End(_) => unreachable!(),
            },
            TokenKind::Special(SpecialToken::PoundSign) => Ast::Decor { token_group_idx },
            TokenKind::Special(_)
            | TokenKind::Identifier(_)
            | TokenKind::WordOpr(_)
            | TokenKind::Literal(_) => self.parse_stmt(token_group_idx, &context),
            TokenKind::Comment => Ast::Comment { token_group_idx },
            TokenKind::Err(_) => todo!(),
        })
    }

    fn parse_if_else_stmts(&mut self, idx: TokenGroupIdx, context: &Context) -> Ast {
        Ast::IfElseStmts {
            if_stmt: self.alloc_stmt(idx, &context),
            elif_stmts: self.alloc_elif_stmts(context.subcontext(AstContextKind::InsideForm)),
            else_stmt: self.alloc_else_stmt(&context),
        }
    }

    fn alloc_elif_stmts(&mut self, context: Context) -> AstIdxRange {
        let mut elif_stmts = vec![];
        while let Some((idx, token_group)) =
            self.token_groups.peek_with_exact_indent(context.indent())
        {
            match token_group.first().kind {
                TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    self.token_groups.next();
                    elif_stmts.push(self.parse_stmt(idx, &context))
                }
                _ => break,
            }
        }
        self.alloc_asts(elif_stmts)
    }

    fn alloc_else_stmt(&mut self, context: &Context) -> Option<AstIdx> {
        let (idx, token_group) = self.token_groups.peek_with_exact_indent(context.indent())?;
        match token_group.first().kind {
            TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                self.token_groups.next();
                Some(self.alloc_stmt(idx, context))
            }
            _ => None,
        }
    }

    fn parse_match_stmts(&mut self, token_group_idx: TokenGroupIdx, context: &Context) -> Ast {
        Ast::MatchStmts {
            pattern_stmt: self.alloc_stmt(token_group_idx, &context),
            case_stmts: self.parse_case_stmts(context.subcontext(AstContextKind::InsideMatchStmt)),
        }
    }

    fn alloc_stmt(&mut self, token_group_idx: TokenGroupIdx, context: &Context) -> AstIdx {
        let ast = self.parse_stmt(token_group_idx, &context);
        self.alloc_ast(ast)
    }

    fn parse_stmt(&mut self, token_group_idx: TokenGroupIdx, context: &Context) -> Ast {
        let body = self.parse_asts(context.subcontext(AstContextKind::InsideForm));
        Ast::Stmt {
            token_group_idx,
            body,
        }
    }

    fn parse_case_stmts(&mut self, context: Context) -> AstIdxRange {
        let mut verticals = vec![];
        while let Some((idx, token_group)) =
            self.token_groups.peek_with_exact_indent(context.indent())
        {
            match token_group.first().kind {
                TokenKind::Special(SpecialToken::Vertical) => {
                    self.token_groups.next();
                    verticals.push(self.parse_stmt(idx, &context))
                }
                _ => break,
            }
        }
        self.alloc_asts(verticals)
    }

    fn parse_defn_or_use(&mut self, token_group_idx: TokenGroupIdx, context: &Context) -> Ast {
        for token in &self.token_sheet[token_group_idx] {
            match token.kind {
                TokenKind::Attr(_) | TokenKind::Comment => (),
                TokenKind::Keyword(Keyword::Use) => {
                    return self.parse_uses(token_group_idx, context)
                }
                TokenKind::Keyword(_) => return self.parse_defn(context, token_group_idx),
                TokenKind::Identifier(_)
                | TokenKind::Special(_)
                | TokenKind::WordOpr(_)
                | TokenKind::Literal(_) => {
                    return Ast::Err {
                        token_group_idx,
                        error: AstError::ExpectDecoratorOrEntityKeyword,
                    }
                }
                TokenKind::Err(_) => todo!(),
            }
        }
        Ast::Err {
            token_group_idx,
            error: todo!(),
        }
    }
}
