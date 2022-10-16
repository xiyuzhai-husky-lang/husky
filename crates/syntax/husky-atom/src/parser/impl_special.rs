use husky_opn_syntax::{BinaryOpr, PureBinaryOpr};

use super::*;

impl<'a, 'b, 'c> AtomParser<'a, 'b, 'c> {
    pub(super) fn handle_special(
        &mut self,
        special: SpecialToken,
        token: &Token,
    ) -> AtomResult<()> {
        let text_start = token.range.start;
        match special {
            SpecialToken::DoubleColon => err!(
                "unexpected double colon, maybe the identifier before is not recognized as entity_route correctly",
                self.token_stream.text_range(text_start)
            )?,
            SpecialToken::DoubleVertical => self.stack.push(HuskyAtom::new(
                self.token_stream.text_range(text_start),
                if !self.stack.is_concave() {
                     BinaryOpr::Pure(PureBinaryOpr::Or).into()
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
                if deprecated_try_eat!(self, SpecialToken::LightArrow) {
                    let output = deprecated_get!(self, ty?);
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
            SpecialToken::RAngle => {
                if let Some(next_token) = self.token_stream.peek() {
                    if next_token.kind == SpecialToken::RAngle.into() {
                        if text_start.j()+1== next_token.range.start.j() {
                            self.token_stream.next();
                            self.stack.push(HuskyAtom::new(
                                self.token_stream.text_range(text_start),
                                BinaryOpr::Pure(PureBinaryOpr::Shr).into(),
                            ))?;
                            return Ok(())
                        }
                    }
                }
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    BinaryOpr::Pure(PureBinaryOpr::Greater).into(),
                ))
            }
            SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Sub)) => {
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    BinaryOpr::Pure(PureBinaryOpr::Sub).into(),
                ))
            }
            SpecialToken::Minus =>{
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    PrefixOpr::Minus.into(),
                ))
            }
            SpecialToken::FieldAccess => {
                let field_ident_token = self
                    .token_stream
                    .next()
                    .ok_or(error!("expect identifier after `.`", self.token_stream.text_range(text_start)))?;
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
                    if let Some(generic_arguments) = deprecated_try_get!(self, angled_generics?) {
                        match self.token_stream.next() {
                            Some(token) => match token.kind {
                                TokenKind::Special(SpecialToken::LPar) => {
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
                    }
                } else {
                    HuskyAtomVariant::FieldAccess(ranged_ident)
                };
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start), atom_variant
                ))
            }
            SpecialToken::QuestionMark => match self.stack.convexity() {
                Convexity::Convex => {
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        RawSuffixOpr::Unveil.into(),
                    ))
                },
                Convexity::Concave => todo!(),
                Convexity::Any => todo!(),
            },
            _ => {
                self.token_stream.text_range(text_start);
                self.stack.push(token.into())
            }
        }
    }
}
