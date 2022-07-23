mod impl_basic;
mod impl_entity_route;
mod impl_lambda_head;
mod impl_parameter;
mod impl_special;
mod impl_state;
mod impl_word_opr;
mod impl_xml;
mod utils;

use super::{stack::AtomStack, *};
use husky_check_utils::should;
use husky_entity_route::{
    EntityKind, EntityRoute, EntityRouteKind, RangedEntityRoute, SpatialArgument,
};
use husky_entity_syntax::EntitySyntaxQueryGroup;
use husky_file::URange;
use husky_print_utils::p;
use husky_text::TextRange;
use husky_token::{
    identify_token, AbsSemanticToken, HuskyToken, HuskyTokenKind, SemanticTokenKind, SpecialToken,
    TokenStream,
};
use std::iter::Peekable;
use utils::*;
use vm::{BinaryOpr, Bracket, PureBinaryOpr};

pub struct AtomParser<'a, 'b> {
    pub atom_context: &'a mut dyn AtomContext,
    pub token_stream: &'a mut TokenStream<'b>,
    stack: AtomStack,
}

impl<'a, 'b> AtomParser<'a, 'b> {
    fn db(&self) -> &dyn EntitySyntaxQueryGroup {
        self.atom_context.entity_syntax_db()
    }

    pub fn new(
        symbol_context: &'a mut dyn AtomContext,
        token_stream: &'a mut TokenStream<'b>,
    ) -> Self {
        Self {
            atom_context: symbol_context,
            token_stream,
            stack: AtomStack::new(),
        }
    }

    pub fn parse_all(mut self) -> AtomResult<Vec<HuskyAtom>> {
        loop {
            if self.stack.is_concave() {
                let text_start = self.token_stream.text_start();
                if let Some(kind) = try_get!(self, symbol?) {
                    self.push(kind, text_start)?;
                }
            }
            let text_start = self.token_stream.text_start();
            if let Some(token) = self.token_stream.next() {
                match token.kind {
                    HuskyTokenKind::Keyword(keyword) => err!(
                        "keyword should be put at start",
                        self.token_stream.text_range(text_start)
                    )?,
                    HuskyTokenKind::Special(SpecialToken::Colon) => {
                        if let Some(_) = self.token_stream.next() {
                            err!("unexpected colon", token.range)?
                        } else {
                            break;
                        }
                    }
                    HuskyTokenKind::Special(special) => self.handle_special(special, token)?,
                    HuskyTokenKind::WordOpr(word_opr) => {
                        self.handle_word_opr(word_opr, text_start)?
                    }
                    HuskyTokenKind::Identifier(_) => err!(
                        "unexpected identifier here",
                        self.token_stream.text_range(text_start)
                    )?,
                    HuskyTokenKind::PrimitiveLiteral(_value) => {
                        let range = self.token_stream.text_range(text_start);
                        self.atom_context
                            .push_abs_semantic_token(AbsSemanticToken::new(
                                SemanticTokenKind::Literal,
                                range,
                            ));
                        self.stack.push(token.into())?
                    }
                    HuskyTokenKind::Unrecognized(c) => {
                        err!(format!("unrecognized char `{}`", c), token.range)?
                    }
                    HuskyTokenKind::IllFormedLiteral(n) => {
                        err!(format!("ill formed literal `{:?}`", n), token.range)?
                    }
                    HuskyTokenKind::Decorator(_) => todo!(),
                }
            } else {
                break;
            }
        }

        if self.stack.is_convex() {
            Ok(self.stack.into())
        } else {
            if let Some(last_atom) = self.stack.atoms.last() {
                err!(format!("last atom is not right convex"), last_atom.range)
            } else {
                Ok(vec![])
            }
        }
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
    tokens: &'a [HuskyToken],
) -> AtomResult<RangedEntityRoute> {
    let result = AtomParser::new(symbol_context, &mut tokens.into()).parse_all()?;
    if result.len() == 0 {
        panic!()
    }
    if result.len() > 1 {
        p!(result);
        err!("too many atoms", result[1..].text_range())?
    } else {
        match result[0].variant {
            AtomVariant::EntityRoute {
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
