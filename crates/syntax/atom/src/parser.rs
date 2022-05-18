mod impl_abs_semantic_token;
mod impl_basic;
mod impl_call_head;
mod impl_entity_route;
mod impl_inner_ops;
mod impl_lambda_head;
mod impl_special;
mod impl_word_opr;
mod utils;

use super::{stack::AtomStack, symbol::SymbolContext, *};
use check_utils::should;
use entity_route::{EntityKind, EntityRoute, EntityRouteKind, GenericArgument};
use print_utils::p;
use std::iter::Peekable;
use text::TextRange;
use token::{identify, AbsSemanticToken, SemanticTokenKind, Special, Token, TokenKind};
use utils::*;
use vm::{BinaryOpr, Bracket, PureBinaryOpr};

#[derive(Debug, Clone)]
pub(crate) struct TokenStream<'a> {
    pub(crate) tokens: &'a [Token],
    start: usize,
    next: usize,
}

impl<'a> TokenStream<'a> {
    pub(crate) fn next(&mut self) -> Option<&'a Token> {
        if self.next < self.tokens.len() {
            let next = self.next;
            self.next += 1;
            Some(&self.tokens[next])
        } else {
            None
        }
    }

    pub(crate) fn next_range(&self) -> TextRange {
        if self.next < self.tokens.len() {
            self.tokens[self.next].range
        } else {
            let last_token_range = self.tokens.last().unwrap().range;
            (last_token_range.end..(last_token_range.end.to_right(4))).into()
        }
    }

    pub(crate) fn pop_range(&mut self) -> TextRange {
        should!(self.start < self.next);
        let start = self.start;
        self.start = self.next;
        self.tokens[start..self.next].text_range()
    }

    pub(crate) fn peek_next_bra(&mut self) -> Option<Bracket> {
        if self.next < self.tokens.len() {
            match self.tokens[self.next].kind {
                TokenKind::Special(special) => special.opt_bra(),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl<'a> From<&'a [Token]> for TokenStream<'a> {
    fn from(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            start: 0,
            next: 0,
        }
    }
}

pub struct AtomParser<'a> {
    symbol_context: &'a SymbolContext<'a>,
    pub(crate) stream: TokenStream<'a>,
    opt_abs_semantic_tokens: Option<&'a mut Vec<AbsSemanticToken>>,
    stack: AtomStack,
}

impl<'a> AtomParser<'a> {
    pub fn new(
        symbol_context: &'a SymbolContext<'a>,
        opt_abs_semantic_tokens: Option<&'a mut Vec<AbsSemanticToken>>,
        tokens: &'a [Token],
    ) -> Self {
        should!(tokens.len() > 0);
        Self {
            symbol_context,
            stream: tokens.into(),
            stack: AtomStack::new(),
            opt_abs_semantic_tokens,
        }
    }

    pub fn parse_all(mut self) -> AtomResult<Vec<Atom>> {
        loop {
            if self.stack.is_concave() {
                if let Some(kind) = try_get!(self, symbol?) {
                    self.push(kind)?;
                }
            }

            if let Some(token) = self.stream.next() {
                match token.kind {
                    TokenKind::Keyword(keyword) => {
                        err!("keyword should be put at start", self.stream.pop_range())?
                    }
                    TokenKind::Special(Special::Colon) => {
                        if let Some(_) = self.stream.next() {
                            err!("unexpected colon", token.range)?
                        } else {
                            break;
                        }
                    }
                    TokenKind::Special(special) => self.handle_special(special, token)?,
                    TokenKind::WordOpr(word_opr) => self.handle_word_opr(word_opr, token)?,
                    TokenKind::Identifier(_) => {
                        err!("unexpected identifier here", self.stream.pop_range())?
                    }
                    TokenKind::PrimitiveLiteral(_value) => {
                        let range = self.stream.pop_range();
                        self.push_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::Literal,
                            range,
                        ));
                        self.stack.push(token.into())?
                    }
                    TokenKind::Unrecognized(c) => {
                        err!(format!("unrecognized char `{}`", c), token.range)?
                    }
                    TokenKind::IllFormedLiteral(n) => {
                        err!(format!("ill formed literal `{:?}`", n), token.range)?
                    }
                    TokenKind::Decorator(_) => todo!(),
                }
            } else {
                break;
            }
        }

        if self.stack.is_convex() {
            Ok(self.stack.into())
        } else {
            let last_atom = self.stack.atoms.last().unwrap();
            err!(format!("last atom is not right convex"), last_atom.range)
        }
    }
}

pub fn parse_route(symbol_context: &SymbolContext, tokens: &[Token]) -> AtomResult<EntityRoutePtr> {
    let result = AtomParser::new(symbol_context, None, tokens.into()).parse_all()?;
    if result.len() == 0 {
        panic!()
    }
    if result.len() > 1 {
        p!(result);
        err!("too many atoms", result[1..].text_range())?
    } else {
        match result[0].kind {
            AtomVariant::EntityRoute {
                route: scope,
                kind: EntityKind::Type(_),
                ..
            } => Ok(scope),
            // AtomKind::ThisType { ty } => Ok(EntityRoutePtr::ThisType),
            _ => err!(
                format!("expect type, but get `{:?}` instead", result[0]),
                result.text_range()
            )?,
        }
    }
}

// pub fn parse_entity(
//     symbol_context: &SymbolContext,
//     tokens: &[Token],
// ) -> AtomResult<EntityRoutePtr> {
//     let result = AtomLRParser::new(symbol_context, tokens.into()).parse_all()?;
//     if result.len() == 0 {
//         panic!()
//     }
//     if result.len() > 1 {
//         p!(result);
//         err!("too many atoms", result[1..].into())?
//     } else {
//         match result[0].kind {
//             AtomKind::EntityRoute { route: scope, .. } => Ok(scope),
//             // AtomKind::ThisType { ty } => Ok(EntityRoutePtr::ThisType),
//             _ => err!(
//                 format!("expect type, but get `{:?}` instead", result[0]),
//                 (&result).into()
//             )?,
//         }
//     }
// }
