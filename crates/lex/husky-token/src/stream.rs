use crate::*;
use husky_opn_syntax::Bracket;
use husky_text::{TextPosition, TextRange};
use parsec::HasParseError;

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    base: usize,
    tokens: &'a [Token],
    next_relative: usize,
}

impl<'a> HasParseError for TokenStream<'a> {
    type Error = TokenError;
}

impl RangedTokenSheet {
    pub fn token_group_token_stream<'a>(
        &'a self,
        token_group_idx: TokenGroupIdx,
        state: Option<TokenIdx>,
    ) -> TokenStream<'a> {
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

#[derive(Debug, PartialEq, Eq)]
pub enum IgnoreComment {
    True,
    False,
}

impl<'a> TokenStream<'a> {
    pub fn is_empty(&self) -> bool {
        self.next_relative >= self.tokens.len()
    }

    pub fn token_position(&self) -> usize {
        self.next_relative
    }

    pub fn save_state(&self) -> TokenIdx {
        TokenIdx(self.base + self.next_relative)
    }

    pub fn try_get_one_token_with_indexed<S>(
        &mut self,
        f: impl Fn(&Token) -> Option<S>,
        ignore_comment: IgnoreComment,
    ) -> Option<(TokenIdx, S)> {
        let (token_idx, token) = self.next_indexed(ignore_comment)?;
        if let Some(s) = f(&token) {
            Some((token_idx, s))
        } else {
            self.go_back();
            None
        }
    }

    pub fn try_eat_with(&mut self, predicate: impl FnOnce(&Token) -> bool) -> Option<&'a Token> {
        let token = self.peek_noncomment_token()?;
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

    pub fn eat_comments(&mut self) {
        while let Some(token) = self.peek_raw() {
            match token {
                Token::Comment => {
                    self.next();
                }
                _ => break,
            }
        }
    }

    pub fn go_back(&mut self) {
        assert!(self.next_relative > 0);
        self.next_relative -= 1;
    }

    pub fn rollback(&mut self, state: TokenIdx) {
        self.next_relative = state.raw() - self.base;
    }

    pub fn next_indexed(&mut self, ignore_comment: IgnoreComment) -> Option<(TokenIdx, &'a Token)> {
        match ignore_comment {
            IgnoreComment::True => {
                while self.next_relative < self.tokens.len() {
                    let next = self.next_relative;
                    self.next_relative += 1;
                    match self.tokens[next] {
                        Token::Comment => continue,
                        _ => return Some((TokenIdx(self.base + next), &self.tokens[next])),
                    }
                }
                None
            }
            IgnoreComment::False => {
                if self.next_relative < self.tokens.len() {
                    let next = self.next_relative;
                    self.next_relative += 1;
                    Some((TokenIdx(self.base + next), &self.tokens[next]))
                } else {
                    None
                }
            }
        }
    }

    pub fn step_back(&mut self) {
        assert!(self.next_relative > 0);
        self.next_relative -= 1
    }

    fn peek_raw(&self) -> Option<&'a Token> {
        if self.next_relative < self.tokens.len() {
            Some(&self.tokens[self.next_relative])
        } else {
            None
        }
    }

    /// this will eat all comments and stop at first noncomment
    pub fn peek_noncomment_token(&mut self) -> Option<&'a Token> {
        self.eat_comments();
        self.peek_raw()
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
        match self.peek_noncomment_token() {
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
    let token_sheet = db.tokenize_snippet(Snippet::new(
        &db,
        "What does a rusty can of spray-on rust remover smell like?\n Irony.".into(),
    ));
    let (token_group_idx, _) = token_sheet.token_group_iter().next().unwrap();
    let mut token_iter = token_sheet.token_group_token_stream(token_group_idx, None);
    while let Some((token_idx, token)) = token_iter.next_indexed(IgnoreComment::False) {
        assert_eq!(&token_sheet[token_idx], token)
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
