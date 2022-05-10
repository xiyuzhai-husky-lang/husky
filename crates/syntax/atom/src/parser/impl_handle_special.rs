use vm::*;

use super::*;

impl<'a> AtomLRParser<'a> {
    pub(super) fn handle_special(&mut self, special: Special, token: &Token) -> AtomResult<()> {
        match special {
            Special::DoubleColon => err!(
                "unexpected double colon, maybe the identifier before is not recognized as scope",
                self.stream.pop_range()
            )?,
            Special::DoubleVertical => self.stack.push(Atom::new(
                self.stream.pop_range(),
                if !self.stack.is_concave() {
                    BinaryOpr::Pure(PureBinaryOpr::BitOr).into()
                } else {
                    AtomVariant::LambdaHead(Vec::new())
                },
            )),
            Special::Vertical => {
                if self.stack.is_concave() {
                    let lambda_head = self.lambda_head()?;
                    self.stack.push(Atom::new(
                        (token.text_start()..self.stream.opt_range.unwrap().end).into(),
                        AtomVariant::LambdaHead(lambda_head),
                    ))
                } else {
                    self.stack.push(Atom::new(
                        self.stream.pop_range(),
                        BinaryOpr::Pure(PureBinaryOpr::BitOr).into(),
                    ))
                }
            }
            Special::Ambersand => self.stack.push(Atom::new(
                self.stream.pop_range(),
                if self.stack.is_concave() {
                    PrefixOpr::Shared.into()
                } else {
                    BinaryOpr::Pure(PureBinaryOpr::BitAnd).into()
                },
            )),
            Special::Exclamation => self.handle_exclamation(),
            Special::LPar => Ok(self.stack.start_list(Bracket::Par, self.stream.pop_range())),
            Special::LBox => Ok(self.stack.start_list(Bracket::Box, self.stream.pop_range())),
            Special::LCurl => Ok(self
                .stack
                .start_list(Bracket::Curl, self.stream.pop_range())),
            Special::RPar => {
                if next_matches!(self, Special::LightArrow) {
                    let output = get!(self, ty?);
                    self.stack
                        .make_func_type(&self.symbol_context, output, self.stream.pop_range())
                } else {
                    self.stack.end_list_or_make_type(
                        Bracket::Par,
                        ListEndAttr::None,
                        self.stream.pop_range(),
                        &self.symbol_context,
                    )
                }
            }
            Special::RBox => self.stack.end_list_or_make_type(
                Bracket::Box,
                ListEndAttr::None,
                self.stream.pop_range(),
                &self.symbol_context,
            ),
            Special::RCurl => self.stack.end_list_or_make_type(
                Bracket::Curl,
                ListEndAttr::None,
                self.stream.pop_range(),
                &self.symbol_context,
            ),
            Special::SubOrMinus => {
                if self.stack.is_convex() {
                    self.stack.push(Atom::new(
                        self.stream.pop_range(),
                        BinaryOpr::Pure(PureBinaryOpr::Sub).into(),
                    ))
                } else {
                    self.stack
                        .push(Atom::new(self.stream.pop_range(), PrefixOpr::Minus.into()))
                }
            }
            Special::MemberAccess => {
                let range = self.stream.pop_range();
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
                self.stack.push(Atom::new(
                    range,
                    SuffixOpr::FieldAccess(ranged_ident).into(),
                ))
            }
            _ => {
                self.stream.pop_range();
                self.stack.push(token.into())
            }
        }
    }

    fn handle_exclamation(&mut self) -> AtomResult<()> {
        let exclamation_range = self.stream.pop_range();
        if let Some(atom_kind) = try_get!(self, symbol?) {
            match atom_kind {
                AtomVariant::EntityRoute { route, kind } => todo!(),
                AtomVariant::Variable { .. }
                | AtomVariant::FrameVariable { .. }
                | AtomVariant::ThisData { .. }
                | AtomVariant::Literal(_) => {
                    self.stack.push(Atom::new(
                        exclamation_range,
                        AtomVariant::Prefix(PrefixOpr::Not),
                    ))?;
                    self.push(atom_kind)
                }
                AtomVariant::Unrecognized(_) => todo!(),
                AtomVariant::Binary(_) => todo!(),
                AtomVariant::Prefix(_) => todo!(),
                AtomVariant::Suffix(_) => todo!(),
                AtomVariant::ListStart(_, _) => todo!(),
                AtomVariant::ListEnd(_, _) => todo!(),
                AtomVariant::ListItem => todo!(),
                AtomVariant::LambdaHead(_) => todo!(),
            }
        } else {
            todo!()
        }
    }
}
