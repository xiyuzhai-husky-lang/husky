mod impl_basic;
mod impl_inner_ops;
mod impl_lambda_head;
mod impl_scope;
mod utils;

use core::slice::Iter;

use common::*;

use scope::{GenericArgument, Scope, ScopeKind, ScopeRoute};
use text::TextRange;
use token::{Special, Token, TokenKind};
use word::CustomIdentifier;

use crate::{
    error::{src, Source},
    scope_proxy::ScopeProxy,
    *,
};

use utils::*;

#[derive(Debug, Clone)]
struct Stream<'a> {
    iter: Iter<'a, Token>,
    range: TextRange,
}

impl<'a> Stream<'a> {
    fn next(&mut self) -> Option<&'a Token> {
        if let Some(token) = self.iter.next() {
            self.range.end = token.text_end();
            Some(token)
        } else {
            None
        }
    }

    fn pop_range(&mut self) -> TextRange {
        let range = self.range.clone();
        self.range = (self.range.end)..(self.range.end);
        range
    }
}

impl<'a> From<&'a [Token]> for Stream<'a> {
    fn from(tokens: &'a [Token]) -> Self {
        Self {
            iter: tokens.iter(),
            range: tokens[0].text_range(),
        }
    }
}

pub struct ScopeLRParser<'a> {
    scope_proxy: ScopeProxy<'a>,
    stream: Stream<'a>,
    atom_group: AtomGroup,
}

impl<'a> ScopeLRParser<'a> {
    pub(super) fn new(
        scope_proxy: ScopeProxy<'a>,
        tokens: &'a [Token],
        atom_group: AtomGroup,
    ) -> Self {
        Self {
            scope_proxy,
            stream: tokens.into(),
            atom_group,
        }
    }

    pub(super) fn parse(mut self) -> AtomParseResult {
        loop {
            if self.atom_group.is_concave() {
                if let Some((scope, kind)) = try_get!(self, scope) {
                    self.push(AtomKind::Scope(scope, kind))?;
                }
            }

            if let Some(token) = self.stream.next() {
                match &token.kind {
                    TokenKind::Keyword(_) => {
                        return atom_err!(token.text_range(), AtomRule::KeywordShouldBeAtStart,)
                    }
                    TokenKind::Special(special) => match special {
                        Special::DoubleColon => todo!(),
                        Special::Colon => {
                            if let Some(_) = self.stream.next() {
                                todo!()
                            } else {
                                break;
                            }
                        }
                        Special::DoubleVertical => self.atom_group.push(Atom::new(
                            token.text_range(),
                            if !self.atom_group.is_concave() {
                                BinaryOpr::BitOr.into()
                            } else {
                                AtomKind::LambdaHead(Vec::new())
                            },
                        ))?,
                        Special::Vertical => {
                            let lambda_head = self.lambda_head()?;
                            self.atom_group.push(Atom::new(
                                token.text_start()..self.stream.range.end,
                                AtomKind::LambdaHead(lambda_head),
                            ));
                        }
                        Special::Ambersand => self.atom_group.push(Atom::new(
                            token.text_range(),
                            if self.atom_group.is_concave() {
                                PrefixOpr::Shared.into()
                            } else {
                                BinaryOpr::BitAnd.into()
                            },
                        ))?,
                        Special::LPar => {
                            self.atom_group.start_list(Bracket::Par, token.text_range())
                        }
                        Special::LBox => {
                            self.atom_group.start_list(Bracket::Box, token.text_range())
                        }
                        Special::LCurl => self
                            .atom_group
                            .start_list(Bracket::Curl, token.text_range()),
                        Special::RPar => {
                            if next_matches!(self, Special::LightArrow) {
                                let output = get!(self, ty?);
                                self.atom_group.make_func_type(
                                    self.scope_proxy,
                                    output,
                                    self.stream.pop_range(),
                                )?;
                            } else {
                                self.atom_group.end_list_or_make_type(
                                    Bracket::Par,
                                    ListEndAttr::None,
                                    token.text_range(),
                                    self.scope_proxy,
                                )?
                            }
                        }
                        Special::RBox => self.atom_group.end_list_or_make_type(
                            Bracket::Box,
                            ListEndAttr::None,
                            token.text_range(),
                            self.scope_proxy,
                        )?,
                        Special::RCurl => self.atom_group.end_list_or_make_type(
                            Bracket::Curl,
                            ListEndAttr::None,
                            token.text_range(),
                            self.scope_proxy,
                        )?,
                        // Special::RPar
                        Special::MemberAccess => todo!(),
                        _ => self.atom_group.push(token.into())?,
                    },
                    _ => self.atom_group.push(token.into())?,
                }
            } else {
                break;
            }
        }
        Ok(self.atom_group)
    }
}
