use super::*;

use husky_opn_syntax::*;
use husky_text::TextLine;

pub(crate) struct Tokenizer<'lex> {
    db: &'lex dyn TokenDb,
    tokens: Vec<Token>,
    token_ranges: Vec<TextRange>,
    line: TextLine,
    comments: Vec<Comment>,
}

enum TokenizerAction {
    Push((Token, TextRange)),
    Comment(TextRange),
    ReplaceLast((Token, TextRange)),
    NewLine,
}

impl<'token> Tokenizer<'token> {
    pub fn new(db: &'token dyn TokenDb) -> Self {
        Self {
            db,
            tokens: vec![],
            token_ranges: vec![],
            line: Default::default(),
            comments: vec![],
        }
    }

    pub fn finish(self) -> RangedTokenSheet {
        RangedTokenSheet::new(self.db, self.tokens, self.token_ranges, self.comments)
    }

    pub(crate) fn push_tokens(&mut self, iter: impl Iterator<Item = RangedPretoken>) {
        for token in iter {
            match self.resolve_token(token) {
                TokenizerAction::Push((token, token_range)) => {
                    self.tokens.push(token);
                    self.token_ranges.push(token_range)
                }
                TokenizerAction::ReplaceLast((token, _token_range)) => {
                    *self.tokens.last_mut().unwrap() = token;
                    todo!()
                }
                TokenizerAction::NewLine => self.line = self.line.to_next_line(),
                TokenizerAction::Comment(range) => {
                    self.comments
                        .push(Comment::new(CommentKind::Todo, self.tokens.len(), range))
                }
            }
        }
    }

    fn resolve_token(&self, ranged_pretoken: RangedPretoken) -> TokenizerAction {
        match ranged_pretoken.token {
            Pretoken::Certain(kind) => TokenizerAction::Push((kind, ranged_pretoken.range)),
            Pretoken::Literal(lit) => match self.tokens.last() {
                Some(Token::Punctuation(Punctuation::Minus)) => TokenizerAction::ReplaceLast((
                    Token::Literal(lit.negative().expect("todo")),
                    ranged_pretoken.range,
                )),
                _ => TokenizerAction::Push((Token::Literal(lit), ranged_pretoken.range)),
            },
            Pretoken::NewLine => TokenizerAction::NewLine,
            Pretoken::Ambiguous(punc) => match punc {
                AmbiguousPretoken::SubOrMinus => {
                    let token = match self.right_convexity() {
                        Convexity::Convex => Token::Punctuation(Punctuation::Binary(
                            BinaryOpr::Closed(BinaryClosedOpr::Sub),
                        )),
                        Convexity::Concave | Convexity::Any => {
                            Token::Punctuation(Punctuation::Minus)
                        }
                    };
                    TokenizerAction::Push((token, ranged_pretoken.range))
                }
                AmbiguousPretoken::For => match self.last_token_in_unfinished_line() {
                    Some(_token) => TokenizerAction::Push((
                        Token::Keyword(ConnectionKeyword::For.into()),
                        ranged_pretoken.range,
                    )),
                    None => TokenizerAction::Push((
                        Token::Keyword(StmtKeyword::NonImplFor.into()),
                        ranged_pretoken.range,
                    )),
                },
            },
            Pretoken::Comment => TokenizerAction::Comment(ranged_pretoken.range),
            Pretoken::Err(e) => TokenizerAction::Push((Token::Error(e), ranged_pretoken.range)),
        }
    }

    fn right_convexity(&self) -> Convexity {
        match self.last_token_in_unfinished_line() {
            Some(token) => token.right_convexity(),
            None => Convexity::Concave,
        }
    }

    fn last_token_in_unfinished_line(&self) -> Option<&Token> {
        if self.token_ranges.last()?.start.line >= self.line {
            self.tokens.last()
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
    //                         if let Some(line) = line_iter.peek().copied() {
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
