use husky_opn_syntax::{BinaryOpr, PureBinaryOpr};

use super::*;

impl<'a, 'b> AtomParser<'a, 'b> {
    pub(super) fn handle_special(
        &mut self,
        special: SpecialToken,
        token: &HuskyToken,
    ) -> AtomResult<()> {
        let text_start = self.token_stream.text_start();
        match special {
            SpecialToken::DoubleColon => err!(
                "unexpected double colon, maybe the identifier before is not recognized as entity_route correctly",
                self.token_stream.text_range(text_start)
            )?,
            SpecialToken::DoubleVertical => self.stack.push(HuskyAtom::new(
                self.token_stream.text_range(text_start),
                if !self.stack.is_concave() {
                    BinaryOpr::Pure(PureBinaryOpr::BitOr).into()
                } else {
                    HuskyAtomVariant::LambdaHead(Vec::new())
                },
            )),
            SpecialToken::Vertical => {
                if self.stack.is_concave() {
                    let lambda_head = self.lambda_head()?;
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        HuskyAtomVariant::LambdaHead(lambda_head),
                    ))
                } else {
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        BinaryOpr::Pure(PureBinaryOpr::BitOr).into(),
                    ))
                }
            }
            SpecialToken::Ambersand => self.stack.push(HuskyAtom::new(
                self.token_stream.text_range(text_start),
                if self.stack.is_concave() {
                    PrefixOpr::Shared.into()
                } else {
                    BinaryOpr::Pure(PureBinaryOpr::BitAnd).into()
                },
            )),
            SpecialToken::Exclamation => self.stack.push(HuskyAtom::new(
                self.token_stream.text_range(text_start),
                PrefixOpr::Not.into(),
            )),
            SpecialToken::LPar => Ok(self
                .stack
                .start_list(Bracket::Par, self.token_stream.text_range(text_start))),
            SpecialToken::LBox => Ok(self
                .stack
                .start_list(Bracket::Box, self.token_stream.text_range(text_start))),
            SpecialToken::LCurl => Ok(self
                .stack
                .start_list(Bracket::Curl, self.token_stream.text_range(text_start))),
            SpecialToken::RPar => {
                if try_eat!(self, SpecialToken::LightArrow) {
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
            SpecialToken::RBox => self.stack.end_list_or_make_type(
                Bracket::Box,
                ListEndAttr::None,
                self.token_stream.text_range(text_start),
                self.atom_context,
            ),
            SpecialToken::RCurl => self.stack.end_list_or_make_type(
                Bracket::Curl,
                ListEndAttr::None,
                self.token_stream.text_range(text_start),
                self.atom_context,
            ),
            SpecialToken::SubOrMinus => {
                if self.stack.is_convex() {
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        BinaryOpr::Pure(PureBinaryOpr::Sub).into(),
                    ))
                } else {
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        PrefixOpr::Minus.into(),
                    ))
                }
            }
            SpecialToken::MemberAccess => {
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
                            HuskyTokenKind::Special(SpecialToken::LPar) => {
                                self.token_stream.text_range(text_start);
                            }
                            _ => todo!(),
                        },
                        None => todo!(),
                    }
                    HuskyAtomVariant::ListStart(
                        Bracket::Par,
                        ListStartAttr::MethodAttach {
                            ranged_ident,
                            generic_arguments,
                        },
                    )
                } else {
                    HuskyAtomVariant::FieldAccess(ranged_ident)
                };
                self.stack.push(HuskyAtom::new(range, atom_variant))
            }
            SpecialToken::QuestionMark => match self.stack.convexity() {
                Convexity::Convex => todo!(),
                Convexity::Concave => todo!(),
            },
            _ => {
                self.token_stream.text_range(text_start);
                self.stack.push(token.into())
            }
        }
    }
}
