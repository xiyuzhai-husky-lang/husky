mod impl_basic;
mod impl_func_head;
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

use crate::{atomic_line_group::AtomStack, scope_proxy::ScopeProxy, *};

pub use utils::*;

#[derive(Debug, Clone)]
pub(crate) struct Stream<'a> {
    pub(crate) iter: Iter<'a, Token>,
    pub(crate) range: TextRange,
}

impl<'a> Stream<'a> {
    pub(crate) fn next(&mut self) -> Option<&'a Token> {
        if let Some(token) = self.iter.next() {
            self.range.end = token.text_end();
            Some(token)
        } else {
            None
        }
    }

    pub(crate) fn pop_range(&mut self) -> TextRange {
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

pub(crate) struct ScopeLRParser<'a> {
    scope_proxy: ScopeProxy<'a>,
    pub(crate) stream: Stream<'a>,
    stack: AtomStack,
}

impl<'a> ScopeLRParser<'a> {
    pub(crate) fn new(scope_proxy: ScopeProxy<'a>, tokens: &'a [Token]) -> Self {
        Self {
            scope_proxy,
            stream: tokens.into(),
            stack: AtomStack::new(),
        }
    }

    pub(crate) fn parse(mut self) -> AtomResult<Vec<Atom>> {
        loop {
            if self.stack.is_concave() {
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
                        Special::DoubleVertical => self.stack.push(Atom::new(
                            token.text_range(),
                            if !self.stack.is_concave() {
                                BinaryOpr::BitOr.into()
                            } else {
                                AtomKind::LambdaHead(Vec::new())
                            },
                        ))?,
                        Special::Vertical => {
                            let lambda_head = self.lambda_head()?;
                            self.stack.push(Atom::new(
                                token.text_start()..self.stream.range.end,
                                AtomKind::LambdaHead(lambda_head),
                            ))?;
                        }
                        Special::Ambersand => self.stack.push(Atom::new(
                            token.text_range(),
                            if self.stack.is_concave() {
                                PrefixOpr::Shared.into()
                            } else {
                                BinaryOpr::BitAnd.into()
                            },
                        ))?,
                        Special::LPar => self.stack.start_list(Bracket::Par, token.text_range()),
                        Special::LBox => self.stack.start_list(Bracket::Box, token.text_range()),
                        Special::LCurl => self.stack.start_list(Bracket::Curl, token.text_range()),
                        Special::RPar => {
                            if next_matches!(self, Special::LightArrow) {
                                let output = get!(self, ty?);
                                self.stack.make_func_type(
                                    self.scope_proxy,
                                    output,
                                    self.stream.pop_range(),
                                )?;
                            } else {
                                self.stack.end_list_or_make_type(
                                    Bracket::Par,
                                    ListEndAttr::None,
                                    token.text_range(),
                                    self.scope_proxy,
                                )?
                            }
                        }
                        Special::RBox => self.stack.end_list_or_make_type(
                            Bracket::Box,
                            ListEndAttr::None,
                            token.text_range(),
                            self.scope_proxy,
                        )?,
                        Special::RCurl => self.stack.end_list_or_make_type(
                            Bracket::Curl,
                            ListEndAttr::None,
                            token.text_range(),
                            self.scope_proxy,
                        )?,
                        Special::SubOrMinus => {
                            if self.stack.is_convex() {
                                self.stack
                                    .push(Atom::new(token.text_range(), BinaryOpr::Sub.into()))?
                            } else {
                                self.stack
                                    .push(Atom::new(token.text_range(), PrefixOpr::Minus.into()))?
                            }
                        }
                        Special::MemberAccess => todo!(),
                        _ => self.stack.push(token.into())?,
                    },
                    _ => self.stack.push(token.into())?,
                }
            } else {
                break;
            }
        }
        Ok(self.stack.into())
    }
}
