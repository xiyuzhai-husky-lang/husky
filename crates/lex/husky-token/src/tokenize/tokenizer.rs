use super::*;

use husky_opn_syntax::*;
use husky_text::TextLine;

pub(crate) struct Tokenizer<'lex> {
    db: &'lex dyn TokenDb,
    tokens: Vec<Token>,
    errors: Vec<TokenError>,
    line: TextLine,
}

enum TokenizerAction {
    Push(Token),
    ReplaceLast(Token),
    NewLine,
}

impl<'token> Tokenizer<'token> {
    pub fn new(db: &'token dyn TokenDb) -> Self {
        Self {
            db,
            tokens: vec![],
            errors: vec![],
            line: Default::default(),
        }
    }

    pub fn finish_with_tokens(self) -> Vec<Token> {
        self.tokens
    }

    pub(crate) fn push_tokens(&mut self, iter: impl Iterator<Item = RangedPretoken>) {
        for token in iter {
            match self.resolve_token(token) {
                TokenizerAction::Push(token) => self.tokens.push(token),
                TokenizerAction::ReplaceLast(token) => *self.tokens.last_mut().unwrap() = token,
                TokenizerAction::NewLine => self.line = self.line.to_next_line(),
            }
        }
    }

    fn resolve_token(&self, token: RangedPretoken) -> TokenizerAction {
        match token.token {
            Pretoken::Certain(kind) => {
                let token = Token {
                    range: token.range,
                    kind,
                };
                TokenizerAction::Push(token)
            }
            Pretoken::Literal(lit) => match self.tokens.last().map(|t| &t.kind) {
                Some(TokenKind::Punctuation(Punctuation::Minus)) => {
                    let token = Token {
                        range: token.range,
                        kind: TokenKind::Literal(lit.negative().expect("todo")),
                    };
                    TokenizerAction::ReplaceLast(token)
                }
                _ => {
                    let token = Token {
                        range: token.range,
                        kind: TokenKind::Literal(lit),
                    };
                    TokenizerAction::Push(token)
                }
            },
            Pretoken::NewLine => TokenizerAction::NewLine,
            Pretoken::Ambiguous(punc) => match punc {
                AmbiguousPretoken::SubOrMinus => {
                    let kind = match self.right_convexity() {
                        Convexity::Convex => TokenKind::Punctuation(Punctuation::Binary(
                            BinaryPunctuation::PureClosed(BinaryPureClosedPunctuation::Sub),
                        )),
                        Convexity::Concave | Convexity::Any => {
                            TokenKind::Punctuation(Punctuation::Minus)
                        }
                    };
                    let token = Token {
                        range: token.range,
                        kind,
                    };
                    TokenizerAction::Push(token)
                }
                AmbiguousPretoken::For => todo!(),
            },
            Pretoken::Comment => TokenizerAction::Push(Token {
                range: token.range,
                kind: TokenKind::Comment,
            }),
            Pretoken::Err(e) => TokenizerAction::Push(Token {
                range: token.range,
                kind: TokenKind::Err(e),
            }),
        }
    }

    fn right_convexity(&self) -> Convexity {
        match self.last_token_in_unfinished_line() {
            Some(token) => token.right_convexity(),
            None => Convexity::Concave,
        }
    }

    fn last_token_in_unfinished_line(&self) -> Option<&Token> {
        let last_token = self.tokens.last()?;
        if last_token.range.start.line >= self.line {
            Some(last_token)
        } else {
            None
        }
    }

    // fn last_token(&self, line: &TokenizedLine) -> &Token {
    //     &self.tokens[line.tokens.end - 1]
    // }

    // fn first_token(&self, line: &TokenizedLine) -> &Token {
    //     &self.tokens[line.tokens.start]
    // }

    // fn produce_line_groups(&mut self) -> Vec<TokenLine> {
    //     todo!()
    //     // let mut line_groups = Vec::new();
    //     // line_groups.reserve_exact(self.tokenized_lines.len());
    //     // let mut line_iter = self
    //     //     .tokenized_lines
    //     //     .iter()
    //     //     .filter(|line| line.tokens.len() > 0)
    //     //     .peekable();
    //     // while let Some(first_line) = line_iter.next() {
    //     //     line_groups.push(
    //     //         unsafe { ref_to_mut_ref(self) }.produce_line_group(first_line, &mut line_iter),
    //     //     );
    //     // }
    //     // line_groups
    // }

    // fn produce_line_group<'a>(
    //     &mut self,
    //     first_line: &TokenizedLine,
    //     line_iter: &mut Peekable<impl Iterator<Item = &'a TokenizedLine>>,
    // ) -> TokenLine {
    //     let group_indent = first_line.indent;
    //     TokenLine {
    //         indent: group_indent,
    //         tokens: TokenIdxRange(
    //             first_line.tokens.start..{
    //                 if self.last_token(first_line).kind == TokenKind::Special(SpecialToken::Colon) {
    //                     if let Some(line) = line_iter.peek() {
    //                         match line.indent.within(group_indent) {
    //                             Ok(is_within) => {
    //                                 if !is_within {
    //                                     self.errors.push(TokenError {
    //                                         message: format!("expect indentated lines after `:`"),
    //                                         range: self.last_token(first_line).range,
    //                                         dev_src: dev_src!(),
    //                                     });
    //                                 }
    //                             }
    //                             Err(e) => self.errors.push(TokenError {
    //                                 message: format!("{:?}", e),
    //                                 range: self.last_token(first_line).range,
    //                                 dev_src: dev_src!(),
    //                             }),
    //                         }
    //                     } else {
    //                         self.errors.push(TokenError {
    //                             message: format!("expect indentated lines after `:`"),
    //                             range: self.last_token(first_line).range,
    //                             dev_src: dev_src!(),
    //                         })
    //                     }
    //                     first_line.tokens.end
    //                 } else {
    //                     loop {
    //                         if let Some(line) = line_iter.peek().map(|e| *e) {
    //                             match line.indent.within(group_indent) {
    //                                 Ok(is_within) => {
    //                                     if is_within {
    //                                         line_iter.next();
    //                                         if self.last_token(line).kind
    //                                             == TokenKind::Special(SpecialToken::Colon)
    //                                         {
    //                                             break line.tokens.end;
    //                                         }
    //                                     } else {
    //                                         fn bind_to_last_line(kind: TokenKind) -> bool {
    //                                             match kind {
    //                                                 TokenKind::Special(special) => match special {
    //                                                     SpecialToken::Ket(_) => true,
    //                                                     _ => false,
    //                                                 },
    //                                                 _ => false,
    //                                             }
    //                                         }

    //                                         if bind_to_last_line(
    //                                             self.first_token(line).kind.clone(),
    //                                         ) {
    //                                             line_iter.next();
    //                                             break line.tokens.end;
    //                                         } else {
    //                                             break line.tokens.start;
    //                                         }
    //                                     }
    //                                 }
    //                                 Err(e) => {
    //                                     self.errors.push(TokenError {
    //                                         message: format!("{:?}", e),
    //                                         range: self.last_token(first_line).range,
    //                                         dev_src: dev_src!(),
    //                                     });
    //                                     line_iter.next();
    //                                 }
    //                             }
    //                         } else {
    //                             break self.tokens.len();
    //                         }
    //                     }
    //                 }
    //             },
    //         ),
    //     }
    // }
}

impl<'lex> std::fmt::Debug for Tokenizer<'lex> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenScanner").finish()
    }
}
