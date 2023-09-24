mod defn;
mod indent;
mod stmt;
mod uses;
mod utils;

use self::indent::Indent;
use crate::*;
use husky_entity_path::DisambiguatorRegistry;
use husky_print_utils::p;
use husky_token::*;
use parsec::{HasStreamState, IsStreamParser, TryParseOptionFromStream};
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
    fn determine_item_kind(_: EntityKindKeywordGroup) -> AstResult<EntityKind>;
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
        let top_level_asts = self.parse_normal_ast_children::<MajorItems>();
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
        let (token_group_idx, token_group, fst, snd) = self
            .token_groups
            .next_token_group_of_no_less_indent_with_its_first_two_tokens(self.indent())?;
        if token_group.indent() > self.indent() {
            return Some(Ast::Err {
                token_group_idx,
                error: OriginalAstError::ExcessiveIndent.into(),
            });
        }
        Some(
            match self.parse_ast_aux::<C>(token_group_idx, token_group, fst, snd) {
                Ok(value) => value,
                Err(error) => Ast::Err {
                    token_group_idx,
                    error,
                },
            },
        )
    }

    fn parse_ast_aux<C: NormalAstChildren>(
        &mut self,
        token_group_idx: TokenGroupIdx,
        token_group: TokenGroup,
        fst: TokenData,
        snd: Option<TokenData>,
    ) -> AstResult<Ast> {
        Ok(match fst {
            TokenData::Keyword(kw) => match kw {
                Keyword::Stmt(kw) => self.try_parse_stmt_after_keyword::<C>(token_group_idx, kw)?,
                Keyword::Todo | Keyword::Sorry | Keyword::Pronoun(_) => {
                    self.try_parse_stmt::<C>(token_group_idx)?
                }
                Keyword::Modifier(_) => Err(OriginalAstError::UnexpectedPattern)?,
                Keyword::Use => self.parse_use_ast(
                    token_group_idx,
                    VisibilityExpr::new_protected(self.module_path),
                    None,
                ),
                Keyword::Mod | Keyword::Fugitive(_) | Keyword::Trait | Keyword::TypeEntity(_) => {
                    self.parse_defn::<C>(
                        token_group_idx,
                        VisibilityExpr::new_protected(self.module_path),
                        None,
                    )
                }
                Keyword::Impl => Ast::ImplBlock {
                    token_group_idx,
                    items: if self.is_trai_impl(token_group_idx) {
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
                    token_group_idx,
                    error: OriginalAstError::UnexpectedEndKeywordAsFirstNonCommentToken.into(),
                },
                Keyword::Connection(_) => todo!(),
                Keyword::Const | Keyword::Pub | Keyword::Static => {
                    self.parse_defn_or_use::<C>(token_group_idx)
                }
                Keyword::Async => todo!(),
            },
            TokenData::Punctuation(Punctuation::POUND) => match snd {
                Some(snd) => match snd {
                    TokenData::Punctuation(Punctuation::LBOX) => Ast::Sorc { token_group_idx },
                    TokenData::Ident(ident) => Ast::Attr {
                        token_group_idx,
                        ident,
                    },
                    _ => todo!(),
                },
                None => todo!(),
            },
            TokenData::Punctuation(_)
            | TokenData::Ident(_)
            | TokenData::Label(_)
            | TokenData::WordOpr(_)
            | TokenData::Literal(_) => self.try_parse_stmt::<C>(token_group_idx)?,
            TokenData::Error(e) => Err(DerivedAstError::TokenData(e))?,
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
            Some(TokenData::Keyword(Keyword::Use)) => self.parse_use_ast(
                token_group_idx,
                visibility_expr,
                Some(aux_parser.finish_with_saved_stream_state()),
            ),
            Some(TokenData::Keyword(_)) => self.parse_defn::<C>(
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
