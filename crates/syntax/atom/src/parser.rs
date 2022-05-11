mod impl_abs_semantic_token;
mod impl_basic;
mod impl_call_head;
mod impl_entity_route;
mod impl_handle_special;
mod impl_inner_ops;
mod impl_lambda_head;
mod utils;

use super::{stack::AtomStack, symbol::SymbolContext, *};
use check_utils::should;
use entity_route::{EntityKind, EntityRoute, EntityRouteKind, GenericArgument};
use print_utils::p;
use std::iter::Peekable;
use text::TextRange;
use token::{identify, AbsSemanticToken, SemanticTokenKind, Special, Token, TokenKind};
use utils::*;
use vm::{BinaryOpr, PureBinaryOpr};

#[derive(Debug, Clone)]
pub(crate) struct TokenStream<'a> {
    pub(crate) iter: Peekable<core::slice::Iter<'a, Token>>,
    opt_range: Option<TextRange>,
}

impl<'a> TokenStream<'a> {
    pub(crate) fn next(&mut self) -> Option<&'a Token> {
        if let Some(token) = self.iter.next() {
            if let Some(range) = &mut self.opt_range {
                range.end = token.text_end()
            } else {
                self.opt_range = Some(token.range)
            }
            Some(token)
        } else {
            None
        }
    }

    pub(crate) fn pop_range(&mut self) -> TextRange {
        let range = self.opt_range.unwrap();
        self.opt_range = None;
        range
    }

    pub(crate) fn is_lpar_next(&mut self) -> bool {
        match self.iter.peek() {
            Some(Token {
                kind: TokenKind::Special(Special::LPar),
                ..
            }) => true,
            _ => false,
        }
    }
}

impl<'a> From<&'a [Token]> for TokenStream<'a> {
    fn from(tokens: &'a [Token]) -> Self {
        Self {
            iter: tokens.iter().peekable(),
            opt_range: Some(tokens.text_range()),
        }
    }
}

pub struct AtomLRParser<'a> {
    symbol_context: &'a SymbolContext<'a>,
    pub(crate) stream: TokenStream<'a>,
    opt_abs_semantic_tokens: Option<&'a mut Vec<AbsSemanticToken>>,
    stack: AtomStack,
}

impl<'a> AtomLRParser<'a> {
    pub fn new(
        symbol_context: &'a SymbolContext<'a>,
        opt_abs_semantic_tokens: Option<&'a mut Vec<AbsSemanticToken>>,
        tokens: &'a [Token],
    ) -> Self {
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
                            todo!()
                        } else {
                            break;
                        }
                    }
                    TokenKind::Special(special) => self.handle_special(special, token)?,
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

pub fn parse_ty(symbol_context: &SymbolContext, tokens: &[Token]) -> AtomResult<EntityRoutePtr> {
    let result = AtomLRParser::new(symbol_context, None, tokens.into()).parse_all()?;
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
