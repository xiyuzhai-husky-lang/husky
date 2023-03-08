use crate::*;

use husky_opn_syntax::Bracket;

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    base: usize,
    tokens: &'a [Token],
    next_relative: usize,
}

impl TokenSheetData {
    pub fn token_group_token_stream<'a>(
        &'a self,
        token_group_idx: TokenGroupIdx,
        state: impl Into<Option<TokenIdx>>,
    ) -> TokenStream<'a> {
        let state: Option<TokenIdx> = state.into();
        let base = self.group_start(token_group_idx);
        let next_relative = state.map(|state| state.raw() - base).unwrap_or_default();
        let tokens = &self[token_group_idx];
        assert!(tokens.len() > 0);
        TokenStream {
            base,
            tokens,
            next_relative,
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<&'a Token> {
        if self.next_relative < self.tokens.len() {
            let next = self.next_relative;
            self.next_relative += 1;
            Some(&self.tokens[next])
        } else {
            None
        }
    }
}

impl<'a> TokenStream<'a> {
    pub fn is_empty(&self) -> bool {
        self.next_relative >= self.tokens.len()
    }

    pub fn token_position(&self) -> usize {
        self.next_relative
    }

    pub fn state(&self) -> TokenIdx {
        TokenIdx(self.base + self.next_relative)
    }

    pub fn try_get_one_token_with_indexed<S>(
        &mut self,
        f: impl Fn(&Token) -> Option<S>,
    ) -> Option<(TokenIdx, S)> {
        let (token_idx, token) = self.next_indexed()?;
        if let Some(s) = f(&token) {
            Some((token_idx, s))
        } else {
            self.go_back();
            None
        }
    }

    pub fn try_eat_with(&mut self, predicate: impl FnOnce(&Token) -> bool) -> Option<&'a Token> {
        let token = self.peek()?;
        if predicate(&token) {
            self.next();
            Some(token)
        } else {
            None
        }
    }

    pub fn try_eat_special(&mut self, punc: Punctuation) -> Option<&'a Token> {
        self.try_eat_with(|token_kind| token_kind == &Token::Punctuation(punc))
    }

    pub fn go_back(&mut self) {
        assert!(self.next_relative > 0);
        self.next_relative -= 1;
    }

    pub fn rollback(&mut self, state: TokenIdx) {
        self.next_relative = state.raw() - self.base;
    }

    pub fn next_index(&self) -> TokenIdx {
        TokenIdx(self.base + self.next_relative)
    }

    pub fn next_indexed(&mut self) -> Option<(TokenIdx, Token)> {
        if self.next_relative < self.tokens.len() {
            let next = self.next_relative;
            self.next_relative += 1;
            Some((TokenIdx(self.base + next), self.tokens[next]))
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

    pub fn peek_next_bra(&mut self) -> Option<Bracket> {
        if self.next_relative < self.tokens.len() {
            match self.tokens[self.next_relative] {
                Token::Punctuation(special) => special.opt_bra(),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn is_next_ident(&mut self) -> bool {
        match self.peek() {
            Some(token) => match token {
                Token::Identifier(_) => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn tokens(&self) -> &[Token] {
        self.tokens
    }
}

#[test]
fn next_indexed_works() {
    let db = DB::default();
    let token_sheet_data = db.snippet_token_sheet_data(Snippet::new(
        &db,
        "What does a rusty can of spray-on rust remover smell like?\n Irony.".into(),
    ));
    let (token_group_idx, _) = token_sheet_data.token_group_iter().next().unwrap();
    let mut token_iter = token_sheet_data.token_group_token_stream(token_group_idx, None);
    while let Some((token_idx, token)) = token_iter.next_indexed() {
        assert_eq!(token_sheet_data[token_idx], token)
    }
}

impl<'a> parsec::HasParseState for TokenStream<'a> {
    // next_relative
    type State = TokenIdx;

    fn save_state(&self) -> Self::State {
        TokenIdx(self.base + self.next_relative)
    }

    fn rollback(&mut self, state: Self::State) {
        self.next_relative = state.raw() - self.base
    }
}
