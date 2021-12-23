mod scope_lr;

use common::*;

use core::{iter::Peekable, slice::Iter};

use file::FileId;
use folded::FoldedList;
use scope::{GenericArgument, InternScope, Scope, ScopeKind};
use text::TextPosition;
use token::{Special, Token, TokenKind, TokenizedText};
use word::CustomIdentifier;

use crate::{
    error::atom_error, kind::LambdaHead, opr::ListStartAttr, parser::scope_lr::ScopeLRParser,
    query::AtomQuery, scope_alias::ScopeAliasStack, *,
};

pub struct AtomParser<'a> {
    db: &'a dyn AtomQuery,
    scope_aliases: ScopeAliasStack,
    folded_results: FoldedList<AtomParseResult>,
}

impl<'a> AtomParser<'a> {
    pub(crate) fn new(db: &'a dyn AtomQuery, module: scope::Module) -> Self {
        Self {
            db,
            scope_aliases: ScopeAliasStack::new(),
            folded_results: FoldedList::new(),
        }
    }

    pub(crate) fn take_folded_results(self) -> FoldedList<AtomParseResult> {
        self.folded_results
    }

    fn try_lr_parse_scope(
        &self,
        token: &Token,
        iter: &mut Peekable<Iter<Token>>,
    ) -> AtomResult<Option<(ScopeId, ScopeKind, TextRange)>> {
        let mut scope_parser = ScopeLRParser::new(self.db, token, iter);
        if let Some((scope, kind)) = scope_parser.try_parse_scope(token)? {
            Ok(Some((scope, kind, scope_parser.range)))
        } else {
            Ok(None)
        }
    }
}

impl<'a> folded::Transformer<'_, [Token], TokenizedText, AtomParseResult, AtomParser<'a>>
    for AtomParser<'a>
{
    fn enter_fold(&mut self) {
        self.scope_aliases.enter_fold();
    }

    fn exit_fold(&mut self) {
        self.scope_aliases.exit_fold();
    }

    fn transform(&mut self, token_group: &[Token]) -> AtomParseResult {
        let mut iter = token_group.iter().peekable();
        let mut atom_group = AtomGroup::new(
            if let TokenKind::Keyword(keyword) = iter.peek().unwrap().kind {
                iter.next();
                Some(keyword)
            } else {
                None
            },
            token_group.last().unwrap().kind == TokenKind::Special(Special::Colon),
        );
        let mut expect_lambda_second_vertical = false;

        while let Some(token) = iter.next() {
            if atom_group.is_concave() {
                if let Some((scope_id, scope_kind, range)) =
                    self.try_lr_parse_scope(token, &mut iter)?
                {
                    atom_group.push(Atom::new(range, AtomKind::Scope(scope_id, scope_kind)))?;
                    continue;
                }
            }
            match &token.kind {
                TokenKind::Keyword(_) => {
                    return atom_err!(token.text_range(), AtomRule::KeywordShouldBeAtStart,)
                }
                TokenKind::Special(special) => match special {
                    Special::DoubleColon => todo!(),
                    Special::Colon => {
                        if None == iter.peek() {
                            return Ok(atom_group);
                        } else {
                            todo!()
                        }
                    }
                    Special::DoubleVertical => {
                        if atom_group.is_convex() {
                            atom_group
                                .push(Atom::new(token.text_range(), BinaryOpr::BitOr.into()))?;
                        } else {
                            atom_group.push(Atom::new(
                                token.text_start()..token.text_start().to_right(1),
                                AtomKind::ListStart(Bracket::Vert, ListStartAttr::None),
                            ))?;
                            atom_group.push(Atom::new(
                                token.text_end().to_left(1)..token.text_end(),
                                AtomKind::ListEnd(Bracket::Vert, ListEndAttr::Attach),
                            ))?;
                        }
                    }
                    Special::Vertical => {
                        if expect_lambda_second_vertical {
                            expect_lambda_second_vertical = false;
                            atom_group.end_lambda(token.text_range());
                        } else {
                            expect_lambda_second_vertical = true;
                            atom_group.start_lambda(token.text_range())?;
                        }
                    }
                    Special::Ambersand => {
                        if atom_group.is_convex() {
                            atom_group
                                .push(Atom::new(token.text_range(), BinaryOpr::BitAnd.into()))?;
                        } else {
                            atom_group
                                .push(Atom::new(token.text_range(), PrefixOpr::Shared.into()))?;
                        }
                    }
                    Special::Exclamation => todo!(),
                    Special::LPar => atom_group.start_list(Bracket::Par, token.text_range()),
                    Special::LBox => atom_group.start_list(Bracket::Box, token.text_range()),
                    Special::LCurl => atom_group.start_list(Bracket::Curl, token.text_range()),
                    Special::RPar => {
                        if let Some(Token {
                            kind: TokenKind::Special(Special::LightArrow),
                            ..
                        }) = iter.peek().map(|e| *e)
                        {
                            let arrow_atom = iter.next().unwrap();
                            let no_type_after_arrow_error = atom_error!(
                                arrow_atom.text_range(),
                                AtomRule::ExpectTypeAfterLightArrow,
                            );
                            let (scope_id, scope_kind, end) = self
                                .try_lr_parse_scope(
                                    iter.next().ok_or(no_type_after_arrow_error.clone())?,
                                    &mut iter,
                                )?
                                .ok_or(no_type_after_arrow_error.clone())?;
                            match scope_kind {
                                ScopeKind::Type { .. } => Ok(()),
                                ScopeKind::Value => Err(no_type_after_arrow_error),
                                ScopeKind::Module => Err(no_type_after_arrow_error),
                                ScopeKind::Func { .. } => Err(no_type_after_arrow_error.clone()),
                                ScopeKind::Trait => Err(no_type_after_arrow_error.clone()),
                            }?;
                            atom_group.make_func_type(self.db, scope_id, arrow_atom.to(&end))?;
                        } else {
                            atom_group.end_list_or_make_type(
                                Bracket::Par,
                                ListEndAttr::None,
                                token.text_range(),
                                self.db,
                            )?
                        }
                    }
                    Special::RBox => atom_group.end_list_or_make_type(
                        Bracket::Box,
                        ListEndAttr::None,
                        token.text_range(),
                        self.db,
                    )?,
                    Special::RCurl => atom_group.end_list_or_make_type(
                        Bracket::Curl,
                        ListEndAttr::None,
                        token.text_range(),
                        self.db,
                    )?,
                    // Special::RPar
                    Special::MemberAccess => todo!(),
                    _ => atom_group.push(token.into())?,
                },
                _ => atom_group.push(token.into())?,
            }
        }

        Ok(atom_group)
    }

    fn folded_results(&mut self) -> &mut FoldedList<AtomParseResult> {
        &mut self.folded_results
    }
}
