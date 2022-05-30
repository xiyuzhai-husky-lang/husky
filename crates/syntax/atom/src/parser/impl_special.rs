use vm::*;

use super::*;

impl<'a, 'b> AtomParser<'a, 'b> {
    pub(super) fn handle_special(&mut self, special: Special, token: &Token) -> AtomResult<()> {
        let text_start = self.token_stream.text_start();
        match special {
            Special::DoubleColon => err!(
                "unexpected double colon, maybe the identifier before is not recognized as scope",
                self.token_stream.text_range(text_start)
            )?,
            Special::DoubleVertical => self.stack.push(Atom::new(
                self.token_stream.text_range(text_start),
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
                        self.token_stream.text_range(text_start),
                        AtomVariant::LambdaHead(lambda_head),
                    ))
                } else {
                    self.stack.push(Atom::new(
                        self.token_stream.text_range(text_start),
                        BinaryOpr::Pure(PureBinaryOpr::BitOr).into(),
                    ))
                }
            }
            Special::Ambersand => self.stack.push(Atom::new(
                self.token_stream.text_range(text_start),
                if self.stack.is_concave() {
                    PrefixOpr::Shared.into()
                } else {
                    BinaryOpr::Pure(PureBinaryOpr::BitAnd).into()
                },
            )),
            Special::Exclamation => self.stack.push(Atom::new(
                self.token_stream.text_range(text_start),
                PrefixOpr::Not.into(),
            )),
            Special::LPar => Ok(self
                .stack
                .start_list(Bracket::Par, self.token_stream.text_range(text_start))),
            Special::LBox => Ok(self
                .stack
                .start_list(Bracket::Box, self.token_stream.text_range(text_start))),
            Special::LCurl => Ok(self
                .stack
                .start_list(Bracket::Curl, self.token_stream.text_range(text_start))),
            Special::RPar => {
                if try_eat!(self, Special::LightArrow) {
                    let output = get!(self, ty?);
                    self.stack.make_func_type(
                        self.atom_context,
                        output,
                        self.token_stream.text_range(text_start),
                    )
                } else {
                    self.stack.end_list_or_make_type(
                        Bracket::Par,
                        ListEndAttr::None,
                        self.token_stream.text_range(text_start),
                        self.atom_context,
                    )
                }
            }
            Special::RBox => self.stack.end_list_or_make_type(
                Bracket::Box,
                ListEndAttr::None,
                self.token_stream.text_range(text_start),
                self.atom_context,
            ),
            Special::RCurl => self.stack.end_list_or_make_type(
                Bracket::Curl,
                ListEndAttr::None,
                self.token_stream.text_range(text_start),
                self.atom_context,
            ),
            Special::SubOrMinus => {
                if self.stack.is_convex() {
                    self.stack.push(Atom::new(
                        self.token_stream.text_range(text_start),
                        BinaryOpr::Pure(PureBinaryOpr::Sub).into(),
                    ))
                } else {
                    self.stack.push(Atom::new(
                        self.token_stream.text_range(text_start),
                        PrefixOpr::Minus.into(),
                    ))
                }
            }
            Special::MemberAccess => {
                let range = self.token_stream.text_range(text_start);
                let field_ident_token = self
                    .token_stream
                    .next()
                    .ok_or(error!("expect identifier after `.`", range))?;
                let is_lpar_or_langle_next = match self.token_stream.peek_next_bra() {
                    Some(Bracket::Par) | Some(Bracket::Angle) => true,
                    _ => false,
                };
                let semantic_token_kind = if is_lpar_or_langle_next {
                    SemanticTokenKind::Method
                } else {
                    SemanticTokenKind::Field
                };
                let ranged_ident = identify_token!(self, field_ident_token, semantic_token_kind);
                let atom_variant = if is_lpar_or_langle_next {
                    let generic_arguments = self.angled_generics()?;
                    match self.token_stream.next() {
                        Some(token) => match token.kind {
                            TokenKind::Special(Special::LPar) => {
                                self.token_stream.text_range(text_start);
                            }
                            _ => todo!(),
                        },
                        None => todo!(),
                    }
                    AtomVariant::ListStart(
                        Bracket::Par,
                        ListStartAttr::MethodAttach {
                            ranged_ident,
                            generic_arguments,
                        },
                    )
                } else {
                    AtomVariant::FieldAccess(ranged_ident)
                };
                self.stack.push(Atom::new(range, atom_variant))
            }
            _ => {
                self.token_stream.text_range(text_start);
                self.stack.push(token.into())
            }
        }
    }
}
