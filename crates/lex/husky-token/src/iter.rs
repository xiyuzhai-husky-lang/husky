use crate::*;
use husky_opn_syntax::Bracket;
use husky_text::{TextPosition, TextRange};

#[derive(Debug, Clone)]
pub struct TokenIter<'a> {
    base: usize,
    tokens: &'a [Token],
    next: usize,
}

impl<'a> TokenIter<'a> {
    pub fn new(base: usize, tokens: &'a [Token]) -> Self {
        TokenIter {
            base,
            tokens,
            next: 0,
        }
    }
}

impl TokenSheet {
    pub fn token_group_token_iter<'a>(&'a self, token_group_idx: TokenGroupIdx) -> TokenIter<'a> {
        let tokens = &self[token_group_idx];
        assert!(tokens.len() > 0);
        TokenIter {
            base: self.group_start(token_group_idx),
            tokens,
            next: 0,
        }
    }
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

    pub fn next_indexed(&mut self) -> Option<(TokenIdx, &'a Token)> {
        if self.next < self.tokens.len() {
            let next = self.next;
            self.next += 1;
            Some((TokenIdx(self.base + next), &self.tokens[next]))
        } else {
            None
        }
    }

    pub fn step_back(&mut self) {
        assert!(self.next > 0);
        self.next -= 1
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

#[test]
fn next_indexed_works() {
    let db = DB::default();
    let tokens = db.tokenize("What does a rusty can of spray-on rust remover smell like?\n Irony.");
    let token_sheet = TokenSheet {
        tokens,
        group_starts: vec![0],
    };
    let (token_group_idx, _) = token_sheet.token_group_iter().next().unwrap();
    let mut token_iter = token_sheet.token_group_token_iter(token_group_idx);
    while let Some((token_idx, token)) = token_iter.next_indexed() {
        assert_eq!(&token_sheet[token_idx], token)
    }
}
