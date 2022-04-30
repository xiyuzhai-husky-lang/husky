use vm::PrimitiveValue;
use word::RangedCustomIdentifier;

use super::*;

impl<'a> AtomLRParser<'a> {
    pub(super) fn handle_special(&mut self, special: Special, token: &Token) -> AtomResult<()> {
        let range = token.text_range();
        match special {
            Special::DoubleColon => err!(
                "unexpected double colon, maybe the identifier before is not recognized as scope",
                range
            )?,
            Special::DoubleVertical => self.stack.push(Atom::new(
                range,
                if !self.stack.is_concave() {
                    BinaryOpr::Pure(PureBinaryOpr::BitOr).into()
                } else {
                    AtomKind::LambdaHead(Vec::new())
                },
            )),
            Special::Vertical => {
                if self.stack.is_concave() {
                    let lambda_head = self.lambda_head()?;
                    self.stack.push(Atom::new(
                        (token.text_start()..self.stream.opt_range.unwrap().end).into(),
                        AtomKind::LambdaHead(lambda_head),
                    ))
                } else {
                    self.stack.push(Atom::new(
                        range,
                        BinaryOpr::Pure(PureBinaryOpr::BitOr).into(),
                    ))
                }
            }
            Special::Ambersand => self.stack.push(Atom::new(
                range,
                if self.stack.is_concave() {
                    PrefixOpr::Shared.into()
                } else {
                    BinaryOpr::Pure(PureBinaryOpr::BitAnd).into()
                },
            )),
            Special::LPar => Ok(self.stack.start_list(Bracket::Par, range)),
            Special::LBox => Ok(self.stack.start_list(Bracket::Box, range)),
            Special::LCurl => Ok(self.stack.start_list(Bracket::Curl, range)),
            Special::RPar => {
                if next_matches!(self, Special::LightArrow) {
                    let output = get!(self, ty?);
                    self.stack
                        .make_func_type(&self.symbol_context, output, self.stream.pop_range())
                } else {
                    self.stack.end_list_or_make_type(
                        Bracket::Par,
                        ListEndAttr::None,
                        range,
                        &self.symbol_context,
                    )
                }
            }
            Special::RBox => self.stack.end_list_or_make_type(
                Bracket::Box,
                ListEndAttr::None,
                range,
                &self.symbol_context,
            ),
            Special::RCurl => self.stack.end_list_or_make_type(
                Bracket::Curl,
                ListEndAttr::None,
                range,
                &self.symbol_context,
            ),
            Special::SubOrMinus => {
                if self.stack.is_convex() {
                    self.stack
                        .push(Atom::new(range, BinaryOpr::Pure(PureBinaryOpr::Sub).into()))
                } else {
                    self.stack.push(Atom::new(range, PrefixOpr::Minus.into()))
                }
            }
            Special::MemberAccess => {
                let field_ident_token = self
                    .stream
                    .next()
                    .ok_or(error!("expect identifier after `.`", range))?;
                let semantic_token_kind = if self.stream.is_lpar_next() {
                    SemanticTokenKind::Method
                } else {
                    SemanticTokenKind::Field
                };
                let ranged_ident = identify!(self, field_ident_token, semantic_token_kind);
                self.stack
                    .push(Atom::new(range, SuffixOpr::MembAccess(ranged_ident).into()))
            }
            _ => self.stack.push(token.into()),
        }
    }
}
