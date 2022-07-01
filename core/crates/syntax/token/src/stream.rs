use crate::*;
use check_utils::should;
use file::URange;
use husky_text::TextPosition;
use vm::Bracket;

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    pub tokens: &'a [Token],
    next: usize,
}

pub struct TokenStreamState {
    next: usize,
}

impl<'a> TokenStream<'a> {
    pub fn empty(&self) -> bool {
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

    pub fn next_range(&self) -> TextRange {
        if self.next < self.tokens.len() {
            self.tokens[self.next].range
        } else {
            let last_token_range = self.tokens.last().unwrap().range;
            (last_token_range.end..(last_token_range.end.to_right(4))).into()
        }
    }

    // pub fn pop_token_slice(&mut self) -> URange {
    //     should!(self.start < self.next);
    //     let start = self.start;
    //     self.start = self.next;
    //     start..self.next
    // }

    // pub fn pop_text_range(&mut self) -> TextRange {
    //     should!(self.start < self.next);
    //     let start = self.start;
    //     self.start = self.next;
    //     self.tokens[start..self.next].text_range()
    // }

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
}

impl<'a> From<&'a [Token]> for TokenStream<'a> {
    fn from(tokens: &'a [Token]) -> Self {
        should!(tokens.len() > 0);
        Self { tokens, next: 0 }
    }
}
