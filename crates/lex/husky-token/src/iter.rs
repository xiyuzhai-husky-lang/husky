use crate::*;
use husky_check_utils::should;
use husky_opn_syntax::Bracket;
use husky_text::{TextPosition, TextRange};

#[derive(Debug, Clone)]
pub struct TokenIter<'a> {
    pub tokens: &'a [Token],
    next: usize,
}

pub struct TokenStreamState {
    next: usize,
}

impl<'a> TokenIter<'a> {
    pub fn is_empty(&self) -> bool {
        self.next >= self.tokens.len()
    }

    pub fn text_start(&self) -> TextPosition {
        if self.next < self.tokens.len() {
            self.tokens[self.next].range.start
        } else {
            self.tokens.last().unwrap().range.end
        }
    }

    fn text_end(&self) -> TextPosition {
        self.tokens[self.next - 1].range.end
    }

    pub fn token_position(&self) -> usize {
        self.next
    }

    pub fn text_range(&self, text_start: TextPosition) -> TextRange {
        (text_start..self.text_end()).into()
    }

    pub fn save_state(&self) -> TokenStreamState {
        TokenStreamState { next: self.next }
    }

    pub fn rollback(&mut self, state: TokenStreamState) {
        self.next = state.next;
    }

    pub fn next(&mut self) -> Option<&'a Token> {
        if self.next < self.tokens.len() {
            let next = self.next;
            self.next += 1;
            Some(&self.tokens[next])
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&'a Token> {
        if self.next < self.tokens.len() {
            Some(&self.tokens[self.next])
        } else {
            None
        }
    }

    pub fn next_range(&self) -> TextRange {
        if self.next < self.tokens.len() {
            self.tokens[self.next].range
        } else {
            let last_token_range = self.tokens.last().unwrap().range;
            (last_token_range.end..(last_token_range.end.to_right(4))).into()
        }
    }

    pub fn peek_next_bra(&mut self) -> Option<Bracket> {
        if self.next < self.tokens.len() {
            match self.tokens[self.next].kind {
                TokenKind::Special(special) => special.opt_bra(),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn is_next_ident(&self) -> bool {
        match self.peek() {
            Some(token) => match token.kind {
                TokenKind::Identifier(_) => true,
                _ => false,
            },
            None => false,
        }
    }
}

impl<'a> From<&'a [Token]> for TokenIter<'a> {
    fn from(tokens: &'a [Token]) -> Self {
        should!(tokens.len() > 0);
        Self { tokens, next: 0 }
    }
}
