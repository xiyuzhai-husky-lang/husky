mod defn;
mod indent;
mod stmt;
mod uses;
mod utils;

use self::indent::Indent;
use crate::*;
use husky_entity_path::DisconnectedConnectionRegistry;

use husky_token::*;

use parsec::{HasStreamState, IsStreamParser};

use utils::*;

pub(crate) struct AstParser<'a> {
    db: &'a ::salsa::Db,
    module_path: ModulePath,
    token_sheet: &'a TokenSheetData,
    token_verses: TokenVerseIter<'a>,
    indent: Indent,
    ast_arena: AstArena,
    disambiguator_registry: DisconnectedConnectionRegistry,
    siblings: Vec<AstIdxRange>,
}

impl<'a> HasStreamState for AstParser<'a> {
    type State = TokenVerseIdx;

    fn save_state(&self) -> Self::State {
        self.token_verses.state()
    }

    fn rollback(&mut self, state: Self::State) {
        self.token_verses.rollback(state)
    }
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(db: &'a ::salsa::Db, module_path: ModulePath) -> Self {
        let token_sheet = db.token_sheet_data(module_path);
        Self {
            db,
            module_path,
            token_sheet,
            token_verses: token_sheet.main_token_verse_iter(),
            indent: Default::default(),
            ast_arena: Default::default(),
            disambiguator_registry: Default::default(),
            siblings: Default::default(),
        }
    }

    pub(crate) fn parse_all(mut self) -> AstSheet {
        let top_level_asts = self.parse_normal_ast_children::<MajorItems>();
        AstSheet::new(self.ast_arena, top_level_asts, self.siblings)
    }

    pub(crate) fn parse_normal_ast_children_indented<C: IsAstChildren>(
        &mut self,
    ) -> Option<AstIdxRange> {
        let range = self.with_indent(|slf| slf.parse_normal_ast_children::<C>());
        (range.len() > 0).then_some(range)
    }

    fn parse_normal_ast_children<C: IsAstChildren>(&mut self) -> AstIdxRange {
        let mut asts: Vec<Ast> = vec![];
        let _token_verse_indices: Vec<TokenVerseIdx> = vec![];
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

    fn parse_ast<C: IsAstChildren>(&mut self) -> Option<Ast> {
        let (token_verse_idx, token_verse, fst, snd) = self
            .token_verses
            .next_token_verse_of_no_less_indent_with_its_first_two_tokens(self.indent())?;
        if token_verse.indent() > self.indent() {
            return Some(Ast::Err {
                token_verse_idx,
                error: OriginalAstError::ExcessiveIndent.into(),
            });
        }
        Some(
            match self.parse_ast_aux::<C>(token_verse_idx, token_verse, fst, snd) {
                Ok(value) => value,
                Err(error) => Ast::Err {
                    token_verse_idx,
                    error,
                },
            },
        )
    }

    fn parse_ast_aux<C: IsAstChildren>(
        &mut self,
        token_verse_idx: TokenVerseIdx,
        _token_verse: TokenVerse,
        fst: TokenData,
        snd: Option<TokenData>,
    ) -> AstResult<Ast> {
        Ok(match fst {
            TokenData::Keyword(kw) => match kw {
                Keyword::Stmt(kw) => self.try_parse_stmt_after_keyword::<C>(token_verse_idx, kw)?,
                Keyword::Todo | Keyword::Sorry | Keyword::Unreachable | Keyword::Pronoun(_) => {
                    self.try_parse_stmt::<C>(token_verse_idx)?
                }
                Keyword::Modifier(_) => Err(OriginalAstError::UnexpectedPattern)?,
                Keyword::Use => self.parse_use_ast(
                    token_verse_idx,
                    VisibilityExpr::new_protected(self.module_path),
                    None,
                ),
                Keyword::Mod | Keyword::Fugitive(_) | Keyword::Trait | Keyword::TypeEntity(_) => {
                    self.parse_defn::<C>(
                        token_verse_idx,
                        VisibilityExpr::new_protected(self.module_path),
                        None,
                    )
                }
                Keyword::Impl => Ast::ImplBlock {
                    token_verse_idx,
                    items: if self.is_trai_impl(token_verse_idx) {
                        // there are no items for marker traits
                        self.try_parse_option::<TraitForTypeItems>()?
                            .map(Into::into)
                    } else {
                        // however, type impl block should always have items
                        Some(
                            self.try_parse_expected::<TypeItems, _>(
                                OriginalAstError::ExpectedTypeItems,
                            )?
                            .into(),
                        )
                    },
                },
                Keyword::End(_) => Ast::Err {
                    token_verse_idx,
                    error: OriginalAstError::UnexpectedEndKeywordAsFirstNonCommentToken.into(),
                },
                Keyword::Connection(_) => Ast::Err {
                    token_verse_idx,
                    error: OriginalAstError::UnexpectedConnectionKeywordAsFirstNonCommentToken
                        .into(),
                },
                Keyword::Const | Keyword::Pub | Keyword::Static => {
                    self.parse_defn_or_use::<C>(token_verse_idx)
                }
            },
            TokenData::Punctuation(Punctuation::POUND) => match snd {
                Some(snd) => match snd {
                    TokenData::Punctuation(Punctuation::LBOX) => Ast::Sorc { token_verse_idx },
                    TokenData::Ident(ident) => Ast::Attr {
                        token_verse_idx,
                        ident,
                    },
                    _ => Ast::Err {
                        token_verse_idx,
                        error: OriginalAstError::ExpectedLboxOrIdentAfterPoundForAttrOrSorce.into(),
                    },
                },
                None => Ast::Err {
                    token_verse_idx,
                    error: OriginalAstError::ExpectedLboxOrIdentAfterPoundForAttrOrSorce.into(),
                },
            },
            TokenData::Punctuation(_)
            | TokenData::Ident(_)
            | TokenData::Label(_)
            | TokenData::WordOpr(_)
            | TokenData::Literal(_) => self.try_parse_stmt::<C>(token_verse_idx)?,
            TokenData::Error(e) => Err(DerivedAstError::TokenData(e))?,
        })
    }

    fn is_trai_impl(&self, token_verse_idx: TokenVerseIdx) -> bool {
        // ad hoc
        // todo: improve this
        self.token_sheet
            .token_verse_token_stream(token_verse_idx, None)
            .find(|token| *token == &(Keyword::Connection(ConnectionKeyword::For).into()))
            .is_some()
    }

    fn parse_defn_or_use<C: IsAstChildren>(&mut self, token_verse_idx: TokenVerseIdx) -> Ast {
        let mut aux_parser = BasicAuxAstParser::new(
            self.db,
            self.module_path,
            self.token_sheet
                .token_verse_token_stream(token_verse_idx, None),
        );
        let visibility_expr = match aux_parser.parse_visibility_expr() {
            Ok(visibility_expr) => visibility_expr,
            Err(e) => {
                return Ast::Err {
                    token_verse_idx,
                    error: e.into(),
                }
            }
        };
        match aux_parser.peek() {
            Some(TokenData::Keyword(Keyword::Use)) => self.parse_use_ast(
                token_verse_idx,
                visibility_expr,
                Some(aux_parser.finish_with_saved_stream_state()),
            ),
            Some(TokenData::Keyword(_)) => self.parse_defn::<C>(
                token_verse_idx,
                visibility_expr,
                Some(aux_parser.finish_with_saved_stream_state()),
            ),
            _ => Ast::Err {
                token_verse_idx,
                error: OriginalAstError::InvalidAstForDefinitionOrUse.into(),
            },
        }
    }
}
