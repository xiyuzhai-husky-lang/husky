use crate::*;
use husky_opn_syntax::Bracket;
use husky_text::{TextPosition, TextRange};

#[derive(Debug, Clone)]
pub struct TokenIter<'a> {
    base: usize,
    tokens: &'a [Token],
    next_relative: usize,
}

impl TokenSheet {
    pub fn token_group_token_iter<'a>(
        &'a self,
        token_group_idx: TokenGroupIdx,
        state: Option<TokenIterState>,
    ) -> TokenIter<'a> {
        let next_relative = state.map(|state| state.next_relative).unwrap_or_default();
        let tokens = &self[token_group_idx];
        assert!(tokens.len() > 0);
        TokenIter {
            base: self.group_start(token_group_idx),
            tokens,
            next_relative,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenIterState {
    next_relative: usize,
}

impl<'a> TokenIter<'a> {
    pub fn is_empty(&self) -> bool {
        self.next_relative >= self.tokens.len()
    }

    pub fn text_start(&self) -> TextPosition {
        if self.next_relative < self.tokens.len() {
            self.tokens[self.next_relative].range.start
        } else {
            self.tokens.last().unwrap().range.end
        }
    }

    fn text_end(&self) -> TextPosition {
        self.tokens[self.next_relative - 1].range.end
    }

    pub fn token_position(&self) -> usize {
        self.next_relative
    }

    pub fn text_range(&self, text_start: TextPosition) -> TextRange {
        (text_start..self.text_end()).into()
    }

    pub fn save_state(&self) -> TokenIterState {
        TokenIterState {
            next_relative: self.next_relative,
        }
    }

    pub fn rollback(&mut self, state: TokenIterState) {
        self.next_relative = state.next_relative;
    }

    pub fn next(&mut self) -> Option<&'a Token> {
        if self.next_relative < self.tokens.len() {
            let next = self.next_relative;
            self.next_relative += 1;
            Some(&self.tokens[next])
        } else {
            None
        }
    }

    pub fn next_indexed(&mut self) -> Option<(TokenIdx, &'a Token)> {
        if self.next_relative < self.tokens.len() {
            let next = self.next_relative;
            self.next_relative += 1;
            Some((TokenIdx(self.base + next), &self.tokens[next]))
        } else {
            None
        }
    }

    pub fn step_back(&mut self) {
        assert!(self.next_relative > 0);
        self.next_relative -= 1
    }

    pub fn peek(&self) -> Option<&'a Token> {
        if self.next_relative < self.tokens.len() {
            Some(&self.tokens[self.next_relative])
        } else {
            None
        }
    }

    pub fn next_range(&self) -> TextRange {
        if self.next_relative < self.tokens.len() {
            self.tokens[self.next_relative].range
        } else {
            let last_token_range = self.tokens.last().unwrap().range;
            (last_token_range.end..(last_token_range.end.to_right(4))).into()
        }
    }

    pub fn peek_next_bra(&mut self) -> Option<Bracket> {
        if self.next_relative < self.tokens.len() {
            match self.tokens[self.next_relative].kind {
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
    let token_sheet = TokenSheet::new(tokens);
    let (token_group_idx, _) = token_sheet.token_group_iter().next().unwrap();
    let mut token_iter = token_sheet.token_group_token_iter(token_group_idx, None);
    while let Some((token_idx, token)) = token_iter.next_indexed() {
        assert_eq!(&token_sheet[token_idx], token)
    }
}

impl<'a> parsec::HasParseState for TokenIter<'a> {
    // next_relative
    type State = usize;

    fn save_state(&self) -> Self::State {
        self.next_relative
    }

    fn rollback(&mut self, state: Self::State) {
        self.next_relative = state
    }
}
