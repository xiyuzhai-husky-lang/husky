mod basic;
mod entity_route;
mod impl_lambda_head;
mod impl_pattern;
mod impl_special;
mod impl_state;
mod impl_word_opr;
mod impl_xml;
mod parameter;
mod pattern;
mod utils;

pub use basic::*;
pub use parameter::*;

use super::{stack::AtomStack, *};
use husky_entity_route::{
    EntityKind, EntityRoute, EntityRouteVariant, RangedEntityRoute, SpatialArgument,
};
use husky_entity_syntax::EntitySyntaxQueryGroup;
use husky_file::URange;
use husky_print_utils::p;
use husky_text::TextRange;
use husky_token::{
    identify_token, AbsSemanticToken, SemanticTokenKind, SpecialToken, Token, TokenKind,
    TokenStream,
};
use pattern::AtomParserPattern;
use utils::*;

pub struct AtomParser<'a, 'b, 'c> {
    pub atom_context: &'a mut dyn AtomContext<'b>,
    pub token_stream: &'a mut TokenStream<'c>,
    stack: AtomStack,
}

impl<'a, 'b, 'c> AtomParser<'a, 'b, 'c> {
    fn db(&self) -> &dyn EntitySyntaxQueryGroup {
        self.atom_context.entity_syntax_db()
    }

    pub fn new(
        symbol_context: &'a mut dyn AtomContext<'b>,
        token_stream: &'a mut TokenStream<'c>,
    ) -> Self {
        Self {
            atom_context: symbol_context,
            token_stream,
            stack: AtomStack::new(),
        }
    }

    pub fn parse_all_atoms(mut self) -> AtomResult<Vec<HuskyAtom>> {
        self.parse_all_remaining_atoms()
    }

    pub fn parse_all_remaining_atoms(&mut self) -> AtomResult<Vec<HuskyAtom>> {
        self.stack.freeze();
        loop {
            if self.stack.is_concave() {
                let text_start = self.token_stream.text_start();
                if let Some(kind) = deprecated_try_get!(self, symbol?) {
                    {
                        self.stack.push(HuskyAtom::new(
                            self.token_stream.text_range(text_start),
                            kind,
                        ))
                    }?;
                }
            }
            let text_start = self.token_stream.text_start();
            if let Some(token) = self.token_stream.next() {
                match token.kind {
                    TokenKind::Keyword(_) => err!(
                        "keyword should be put at start",
                        self.token_stream.text_range(text_start)
                    )?,
                    TokenKind::Special(SpecialToken::Colon) => {
                        if let Some(_) = self.token_stream.next() {
                            err!("unexpected colon", token.range)?
                        } else {
                            break;
                        }
                    }
                    TokenKind::Special(special) => self.handle_special(special, token)?,
                    TokenKind::WordOpr(word_opr) => self.handle_word_opr(word_opr, text_start)?,
                    TokenKind::Identifier(_) => err!(
                        "unexpected identifier here",
                        self.token_stream.text_range(text_start)
                    )?,
                    TokenKind::PrimitiveLiteral(_value) => {
                        let range = self.token_stream.text_range(text_start);
                        self.atom_context
                            .push_abs_semantic_token(AbsSemanticToken::new(
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
                    TokenKind::WordPattern(_) => {
                        let range = self.token_stream.text_range(text_start);
                        self.atom_context
                            .push_abs_semantic_token(AbsSemanticToken::new(
                                SemanticTokenKind::WordPattern,
                                range,
                            ));
                        self.stack.push(token.into())?
                    }
                }
            } else {
                break;
            }
        }
        self.stack.unfreeze()
    }

    pub fn try_get<P: AtomParserPattern>(&mut self, patt: &P) -> AtomResult<Option<P::Output>> {
        let saved_state = self.save_state();
        Ok(if let Some(pattern) = patt.get_parsed(self)? {
            Some(pattern)
        } else {
            self.rollback(saved_state);
            None
        })
    }

    pub fn try_eat<P: AtomParserPattern>(&mut self, patt: &P) -> AtomResult<bool> {
        let saved_state = self.save_state();
        Ok(if patt.get_parsed(self)?.is_some() {
            true
        } else {
            self.rollback(saved_state);
            false
        })
    }

    fn push_abs_semantic_token(&mut self, new_token: AbsSemanticToken) {
        self.atom_context.push_abs_semantic_token(new_token)
    }

    pub fn push_symbol(&mut self, f: impl FnOnce(&mut dyn AtomContext) -> Symbol) {
        let new_symbol = f(self.atom_context);
        self.atom_context.push_symbol(new_symbol)
    }
}

pub fn parse_route<'a, 'b>(
    symbol_context: &'a mut dyn AtomContext,
    tokens: &'a [Token],
) -> AtomResult<RangedEntityRoute> {
    let result = AtomParser::new(symbol_context, &mut tokens.into()).parse_all_remaining_atoms()?;
    if result.len() == 0 {
        panic!()
    }
    if result.len() > 1 {
        p!(result);
        err!("too many atoms", result[1..].text_range())?
    } else {
        match result[0].variant {
            HuskyAtomVariant::EntityRoute {
                route,
                kind: EntityKind::Type(_),
                ..
            } => Ok(RangedEntityRoute {
                route,
                range: tokens.text_range(),
            }),
            // AtomKind::ThisType { ty } => Ok(EntityRoutePtr::ThisType),
            _ => err!(
                format!("expect type, but get `{:?}` instead", result[0]),
                result.text_range()
            )?,
        }
    }
}

// pub fn parse_entity(
//     symbol_context: &mut SymbolContext,
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
