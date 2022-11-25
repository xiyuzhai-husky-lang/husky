use crate::*;
use husky_dev_utils::dev_src;
use husky_opn_syntax::*;
use husky_text::TextIndent;
use husky_token::*;
use std::{iter::Peekable, sync::Arc};

pub(crate) struct Tokenizer<'lex> {
    db: &'lex dyn IdentifierDb,
    tokens: Vec<Token>,
    tokenized_lines: Vec<TokenizedLine>,
    errors: Vec<LexError>,
}

#[derive(PartialEq, Eq)]
pub struct TokenizedLine {
    pub(crate) indent: TextIndent,
    pub(crate) tokens: TokenGroup,
}

impl std::fmt::Debug for TokenizedLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "TokenizedLine{{Indent({:?}), tokens: {:?}}}",
            &self.indent.get_raw(),
            &self.tokens,
        ))
    }
}

enum TokenizerAction {
    Push,
    ReplaceLast,
}

impl<'token> Tokenizer<'token> {
    pub fn new(db: &'token dyn IdentifierDb) -> Self {
        Self {
            db,
            tokens: vec![],
            tokenized_lines: vec![],
            errors: vec![],
        }
    }

    pub fn finish_with_tokens(self) -> Vec<Token> {
        self.tokens
    }

    pub fn scan_line(&mut self, line_index: usize, line: &str) {
        let start = self.tokens.len();
        let (indent, token_iter) =
            RawTokenIter::new(self.db, line_index, line.chars().enumerate().peekable());
        self.push_tokens(token_iter);
        let end = self.tokens.len();
        self.tokenized_lines.push(TokenizedLine {
            indent,
            tokens: TokenGroup(start..end),
        })
    }

    fn push_tokens(&mut self, iter: impl Iterator<Item = RawToken>) {
        for token in iter {
            let (action, token) = self.resolve_token(token);
            match action {
                TokenizerAction::Push => self.tokens.push(token),
                TokenizerAction::ReplaceLast => *self.tokens.last_mut().unwrap() = token,
            }
        }
    }

    fn resolve_token(&self, token: RawToken) -> (TokenizerAction, Token) {
        let (action, kind) = match token.kind {
            RawTokenKind::Certain(token_kind) => (TokenizerAction::Push, token_kind),
            RawTokenKind::SubOrMinus => (
                TokenizerAction::Push,
                match self.right_convexity() {
                    Convexity::Convex => TokenKind::Special(SpecialToken::BinaryOpr(
                        BinaryOpr::PureClosed(BinaryPureClosedOpr::Sub),
                    )),
                    Convexity::Concave | Convexity::Any => TokenKind::Special(SpecialToken::Minus),
                },
            ),
            RawTokenKind::Literal(lit) => match self.tokens.last().map(|t| t.kind) {
                Some(TokenKind::Special(SpecialToken::Minus)) => (
                    TokenizerAction::ReplaceLast,
                    TokenKind::Literal(lit.negative().expect("todo")),
                ),
                _ => (TokenizerAction::Push, TokenKind::Literal(lit)),
            },
            RawTokenKind::IllFormedLiteral(l) => {
                (TokenizerAction::Push, TokenKind::IllFormedLiteral(l))
            }
        };
        (
            action,
            Token {
                range: token.range,
                kind,
            },
        )
    }

    fn right_convexity(&self) -> Convexity {
        match self.last_token_in_unfinished_line() {
            Some(token) => token.right_convexity(),
            None => Convexity::Concave,
        }
    }

    fn last_token_in_unfinished_line(&self) -> Option<&Token> {
        match self.tokenized_lines.last() {
            Some(line) => {
                if line.tokens.end == self.tokens.len() {
                    None
                } else {
                    assert!(line.tokens.end < self.tokens.len());
                    self.tokens.last()
                }
            }
            None => self.tokens.last(),
        }
    }

    fn last_token(&self, line: &TokenizedLine) -> &Token {
        &self.tokens[line.tokens.end - 1]
    }

    fn first_token(&self, line: &TokenizedLine) -> &Token {
        &self.tokens[line.tokens.start]
    }

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
    //                                     self.errors.push(LexError {
    //                                         message: format!("expect indentated lines after `:`"),
    //                                         range: self.last_token(first_line).range,
    //                                         dev_src: dev_src!(),
    //                                     });
    //                                 }
    //                             }
    //                             Err(e) => self.errors.push(LexError {
    //                                 message: format!("{:?}", e),
    //                                 range: self.last_token(first_line).range,
    //                                 dev_src: dev_src!(),
    //                             }),
    //                         }
    //                     } else {
    //                         self.errors.push(LexError {
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
    //                                     self.errors.push(LexError {
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
