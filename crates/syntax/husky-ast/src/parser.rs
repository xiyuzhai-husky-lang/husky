mod context;
mod defn;
mod uses;
mod utils;

use crate::*;
use context::*;
use husky_entity_path::DisambiguatorRegistry;
use husky_print_utils::p;
use husky_token::{
    ConnectionKeyword, Keyword, Punctuation, RangedTokenSheet, StmtKeyword, Token, TokenGroupIter,
    TokenSheetData,
};
use utils::*;

pub(crate) struct AstParser<'a> {
    db: &'a dyn AstDb,
    module_path: ModulePath,
    token_sheet: &'a TokenSheetData,
    token_groups: TokenGroupIter<'a>,
    ast_arena: AstArena,
    disambiguator_registry: DisambiguatorRegistry,
    siblings: Vec<AstIdxRange>,
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(db: &'a dyn AstDb, module_path: ModulePath) -> VfsResult<Self> {
        let token_sheet = db.token_sheet_data(module_path)?;
        Ok(Self {
            db,
            module_path,
            token_sheet,
            token_groups: token_sheet.token_group_iter(),
            ast_arena: Default::default(),
            disambiguator_registry: Default::default(),
            siblings: Default::default(),
        })
    }

    pub(crate) fn parse_all(mut self) -> AstSheet {
        let top_level_asts = self.parse_asts(Context::new_module());
        AstSheet::new(self.ast_arena, top_level_asts, self.siblings)
    }

    fn parse_asts(&mut self, context: Context) -> AstIdxRange {
        let mut asts: Vec<Ast> = vec![];
        let _token_group_indices: Vec<TokenGroupIdx> = vec![];
        while let Some(ast) = self.parse_ast(&context) {
            asts.push(ast)
        }
        let ast_idx_range = self.alloc_asts(asts);
        self.siblings.push(ast_idx_range);
        ast_idx_range
    }

    fn alloc_asts(&mut self, asts: Vec<Ast>) -> AstIdxRange {
        self.ast_arena.alloc_batch(asts)
    }

    fn alloc_ast(&mut self, ast: Ast) -> AstIdx {
        self.ast_arena.alloc_one(ast)
    }

    fn parse_ast(&mut self, context: &Context) -> Option<Ast> {
        let (token_group_idx, token_group, first_token) = self
            .token_groups
            .next_token_group_of_no_less_indent_with_its_first_token(context.indent())?;
        if token_group.indent() > context.indent() {
            return Some(Ast::Err {
                token_group_idx,
                error: OriginalAstError::ExcessiveIndent.into(),
            });
        }
        Some(match first_token {
            Token::Keyword(kw) => match kw {
                Keyword::Stmt(kw) => {
                    match context.kind().allow_stmt() {
                        Ok(_) => (),
                        Err(error) => {
                            return Some(Ast::Err {
                                token_group_idx,
                                error,
                            })
                        }
                    }
                    match kw {
                        StmtKeyword::If => self.parse_if_else_stmts(token_group_idx, &context),
                        StmtKeyword::Elif => Ast::Err {
                            token_group_idx,
                            error: OriginalAstError::StandaloneElif.into(),
                        },
                        StmtKeyword::Else => Ast::Err {
                            token_group_idx,
                            error: OriginalAstError::StandaloneElse.into(),
                        },
                        StmtKeyword::Match => self.parse_match_stmts(token_group_idx, &context),
                        StmtKeyword::While
                        | StmtKeyword::Do
                        | StmtKeyword::NonImplFor
                        | StmtKeyword::ForExt
                        | StmtKeyword::Let
                        | StmtKeyword::Break
                        | StmtKeyword::Return
                        | StmtKeyword::Assert
                        | StmtKeyword::Require => self.parse_stmt(token_group_idx, &context),
                    }
                }
                Keyword::Pronoun(_) => self.parse_stmt(token_group_idx, &context),
                Keyword::Pattern(_) => {
                    return Some(Ast::Err {
                        token_group_idx,
                        error: OriginalAstError::UnexpectedPattern.into(),
                    });
                }
                Keyword::Use => self.parse_use_ast(
                    token_group_idx,
                    context,
                    VisibilityExpr::new_protected(self.module_path),
                    None,
                ),
                Keyword::Main => Ast::Main {
                    token_group_idx,
                    body: self.parse_asts(context.subcontext(AstContextKind::InsideForm)),
                },
                Keyword::Config(_) => Ast::Config {
                    token_group_idx,
                    body: self.parse_asts(context.subcontext(AstContextKind::InsideForm)),
                },
                Keyword::Mod
                | Keyword::Form(_)
                | Keyword::Visual
                | Keyword::Trait
                | Keyword::TypeEntity(_) => self.parse_defn(
                    context,
                    token_group_idx,
                    VisibilityExpr::new_protected(self.module_path),
                    None,
                ),
                Keyword::Impl => Ast::Impl {
                    token_group_idx,
                    body: self.parse_asts(context.subcontext(
                        if self.is_trai_impl(token_group_idx) {
                            AstContextKind::InsideTraitForTypeImplBlock
                        } else {
                            AstContextKind::InsideTypeImplBlock
                        },
                    )),
                },
                Keyword::End(_) => Ast::Err {
                    token_group_idx,
                    error: OriginalAstError::UnexpectedEndKeywordAsFirstNonCommentToken.into(),
                },
                Keyword::Connection(_) => todo!(),
                Keyword::Pub | Keyword::Static => self.parse_defn_or_use(token_group_idx, context),
                Keyword::Async => todo!(),
            },
            Token::Punctuation(Punctuation::POUND) => Ast::Attr { token_group_idx },
            Token::Punctuation(Punctuation::AT) => match token_group.second() {
                Some(Token::Ident(ident)) => Ast::Decr {
                    token_group_idx,
                    ident,
                },
                _ => todo!(),
            },
            Token::Punctuation(_)
            | Token::Ident(_)
            | Token::Label(_)
            | Token::WordOpr(_)
            | Token::Literal(_) => self.parse_stmt(token_group_idx, &context),
            Token::Error(e) => Ast::Err {
                token_group_idx,
                error: DerivedAstError::Token(e).into(),
            },
        })
    }

    fn is_trai_impl(&self, token_group_idx: TokenGroupIdx) -> bool {
        // ad hoc
        // todo: improve this
        self.token_sheet
            .token_group_token_stream(token_group_idx, None)
            .find(|token| *token == &(Keyword::Connection(ConnectionKeyword::For).into()))
            .is_some()
    }

    fn parse_if_else_stmts(&mut self, idx: TokenGroupIdx, context: &Context) -> Ast {
        Ast::IfElseStmts {
            if_branch: self.alloc_stmt(idx, context),
            elif_branches: self.alloc_elif_stmts(context),
            else_branch: self.alloc_else_stmt(context),
        }
    }

    fn alloc_elif_stmts(&mut self, context: &Context) -> AstIdxRange {
        let mut elif_stmts = vec![];
        while let Some((idx, token_group, first_noncomment_token)) = self
            .token_groups
            .peek_token_group_of_exact_indent_with_its_first_token(context.indent())
        {
            match first_noncomment_token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    self.token_groups.next();
                    elif_stmts.push(self.parse_stmt(idx, &context))
                }
                _ => break,
            }
        }
        self.alloc_asts(elif_stmts)
    }

    fn alloc_else_stmt(&mut self, context: &Context) -> Option<AstIdx> {
        let (idx, token_group, first_noncomment_token) = self
            .token_groups
            .peek_token_group_of_exact_indent_with_its_first_token(context.indent())?;
        match first_noncomment_token {
            Token::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                self.token_groups.next();
                Some(self.alloc_stmt(idx, context))
            }
            _ => None,
        }
    }

    fn parse_match_stmts(&mut self, token_group_idx: TokenGroupIdx, context: &Context) -> Ast {
        Ast::MatchStmts {
            token_group_idx,
            pattern_stmt: self.alloc_stmt(token_group_idx, &context),
            case_stmts: self.parse_case_stmts(context.subcontext(AstContextKind::InsideMatchStmt)),
        }
    }

    fn alloc_stmt(&mut self, token_group_idx: TokenGroupIdx, context: &Context) -> AstIdx {
        let ast = self.parse_stmt(token_group_idx, &context);
        self.alloc_ast(ast)
    }

    fn parse_stmt(&mut self, token_group_idx: TokenGroupIdx, context: &Context) -> Ast {
        match context.kind().allow_stmt() {
            Ok(_) => (),
            Err(error) => {
                return Ast::Err {
                    token_group_idx,
                    error,
                }
            }
        }
        let body = self.parse_asts(context.subcontext(AstContextKind::InsideForm));
        Ast::BasicStmtOrBranch {
            token_group_idx,
            body,
        }
    }

    fn parse_case_stmts(&mut self, context: Context) -> AstIdxRange {
        let mut verticals = vec![];
        while let Some((idx, token_group, first)) = self
            .token_groups
            .peek_token_group_of_exact_indent_with_its_first_token(context.indent())
        {
            match first {
                Token::Punctuation(Punctuation::VERTICAL) => {
                    self.token_groups.next();
                    verticals.push(self.parse_stmt(idx, &context))
                }
                _ => break,
            }
        }
        self.alloc_asts(verticals)
    }

    fn parse_defn_or_use(&mut self, token_group_idx: TokenGroupIdx, ctx: &Context) -> Ast {
        // for token in &self.token_sheet[token_group_idx] {
        //     match token {
        //         Token::Keyword(Keyword::Pub | Keyword::Pronoun(_)) => (),
        //         Token::Keyword(Keyword::Use) => {
        //             return self.parse_use_ast(token_group_idx, context)
        //         }
        //         Token::Keyword(_) => return self.parse_defn(context, token_group_idx, todo!()),
        //         _ => (),
        //     }
        // }
        let mut aux_parser = BasicAuxAstParser::new(
            self.db,
            ctx,
            self.module_path,
            self.token_sheet
                .token_group_token_stream(token_group_idx, None),
        );
        let visibility_expr = match aux_parser.parse_visibility_expr() {
            Ok(visibility_expr) => visibility_expr,
            Err(e) => {
                return Ast::Err {
                    token_group_idx,
                    error: e.into(),
                }
            }
        };
        match aux_parser.peek() {
            Some(Token::Keyword(Keyword::Use)) => self.parse_use_ast(
                token_group_idx,
                ctx,
                visibility_expr,
                Some(aux_parser.finish_with_saved_stream_state()),
            ),
            Some(Token::Keyword(_)) => self.parse_defn(
                ctx,
                token_group_idx,
                visibility_expr,
                Some(aux_parser.finish_with_saved_stream_state()),
            ),
            _ => Ast::Err {
                token_group_idx,
                error: OriginalAstError::InvalidAstForDefinitionOrUse.into(),
            },
        }
    }
}
