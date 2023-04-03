mod defn;
mod indent;
mod uses;
mod utils;

use self::indent::Indent;
use crate::*;
use husky_entity_path::DisambiguatorRegistry;
use husky_print_utils::p;
use husky_token::*;
use parsec::{HasStreamState, ParseFromStreamWithError, StreamParser};
use salsa::DebugWithDb;
use utils::*;

pub(crate) struct AstParser<'a> {
    db: &'a dyn AstDb,
    module_path: ModulePath,
    token_sheet: &'a TokenSheetData,
    token_groups: TokenGroupIter<'a>,
    indent: Indent,
    ast_arena: AstArena,
    disambiguator_registry: DisambiguatorRegistry,
    siblings: Vec<AstIdxRange>,
}

pub(crate) trait NormalAstChildren {
    const ALLOW_STMT: AstResult<()>;
    fn determine_entity_kind(_: EntityKeywordGroup) -> AstResult<EntityKind>;
}

impl<'a> HasStreamState for AstParser<'a> {
    type State = TokenGroupIdx;

    fn save_state(&self) -> Self::State {
        self.token_groups.state()
    }

    fn rollback(&mut self, state: Self::State) {
        self.token_groups.rollback(state)
    }
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(db: &'a dyn AstDb, module_path: ModulePath) -> VfsResult<Self> {
        let token_sheet = db.token_sheet_data(module_path)?;
        Ok(Self {
            db,
            module_path,
            token_sheet,
            token_groups: token_sheet.token_group_iter(),
            indent: Default::default(),
            ast_arena: Default::default(),
            disambiguator_registry: Default::default(),
            siblings: Default::default(),
        })
    }

    pub(crate) fn parse_all(mut self) -> AstSheet {
        let top_level_asts = self.parse_normal_ast_children::<ModuleItems>();
        AstSheet::new(self.ast_arena, top_level_asts, self.siblings)
    }

    pub(crate) fn parse_normal_ast_children_indented<C: NormalAstChildren>(
        &mut self,
    ) -> Option<AstIdxRange> {
        let range = self.with_indent(|this| this.parse_normal_ast_children::<C>());
        (range.len() > 0).then_some(range)
    }

    fn parse_normal_ast_children<C: NormalAstChildren>(&mut self) -> AstIdxRange {
        let mut asts: Vec<Ast> = vec![];
        let _token_group_indices: Vec<TokenGroupIdx> = vec![];
        while let Some(ast) = self.parse_ast::<C>() {
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

    fn parse_ast<C: NormalAstChildren>(&mut self) -> Option<Ast> {
        let (token_group_idx, token_group, first_token) = self
            .token_groups
            .next_token_group_of_no_less_indent_with_its_first_token(self.indent())?;
        if token_group.indent() > self.indent() {
            return Some(Ast::Err {
                token_group_idx,
                error: OriginalAstError::ExcessiveIndent.into(),
            });
        }
        Some(match first_token {
            Token::Keyword(kw) => match kw {
                Keyword::Stmt(kw) => {
                    match C::ALLOW_STMT {
                        Ok(_) => (),
                        Err(error) => {
                            return Some(Ast::Err {
                                token_group_idx,
                                error,
                            })
                        }
                    }
                    match kw {
                        StmtKeyword::If => self.parse_if_else_stmts(token_group_idx),
                        StmtKeyword::Elif => Ast::Err {
                            token_group_idx,
                            error: OriginalAstError::StandaloneElif.into(),
                        },
                        StmtKeyword::Else => Ast::Err {
                            token_group_idx,
                            error: OriginalAstError::StandaloneElse.into(),
                        },
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
                    }
                }
                Keyword::Pronoun(_) => self.parse_stmt(token_group_idx),
                Keyword::Pattern(_) => {
                    return Some(Ast::Err {
                        token_group_idx,
                        error: OriginalAstError::UnexpectedPattern.into(),
                    });
                }
                Keyword::Use => self.parse_use_ast(
                    token_group_idx,
                    VisibilityExpr::new_protected(self.module_path),
                    None,
                ),
                Keyword::Main => Ast::Main {
                    token_group_idx,
                    body: todo!(), //  self.parse_asts(context.subcontext(AstContextKind::InsideForm)),
                },
                Keyword::Config(_) => Ast::Config {
                    token_group_idx,
                    body: todo!(), // self.parse_asts(context.subcontext(AstContextKind::InsideForm)),
                },
                Keyword::Mod
                | Keyword::Form(_)
                | Keyword::Visual
                | Keyword::Trait
                | Keyword::TypeEntity(_) => self.parse_defn::<C>(
                    token_group_idx,
                    VisibilityExpr::new_protected(self.module_path),
                    None,
                ),
                Keyword::Impl => {
                    let items = if self.is_trai_impl(token_group_idx) {
                        self.parse_expected(OriginalAstError::ExpectedTraitForTypeItems)
                            .map(ImplBlockItems::TraitForType)
                    } else {
                        self.parse_expected(OriginalAstError::ExpectedTypeItems)
                            .map(ImplBlockItems::Type)
                    };
                    match items {
                        Ok(items) => Ast::ImplBlock {
                            token_group_idx,
                            items,
                        },
                        Err(error) => Ast::Err {
                            token_group_idx,
                            error,
                        },
                    }
                }
                Keyword::End(_) => Ast::Err {
                    token_group_idx,
                    error: OriginalAstError::UnexpectedEndKeywordAsFirstNonCommentToken.into(),
                },
                Keyword::Connection(_) => todo!(),
                Keyword::Pub | Keyword::Static => self.parse_defn_or_use::<C>(token_group_idx),
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
            | Token::Literal(_) => self.parse_stmt(token_group_idx),
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
        Ast::BasicStmtOrBranch {
            token_group_idx,
            body: todo!(),
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

    fn parse_defn_or_use<C: NormalAstChildren>(&mut self, token_group_idx: TokenGroupIdx) -> Ast {
        let mut aux_parser = BasicAuxAstParser::new(
            self.db,
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
                visibility_expr,
                Some(aux_parser.finish_with_saved_stream_state()),
            ),
            Some(Token::Keyword(_)) => self.parse_defn::<C>(
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
