#![feature(trait_upcasting)]
#[cfg(test)]
mod tests;
mod token;
mod token_group;
mod token_idx;

pub use self::token::*;
pub use self::token_group::*;
pub use self::token_idx::*;

#[cfg(test)]
use crate::tests::*;
use husky_coword::Ident;
use husky_opr::Bracket;
use husky_token::TokenGroupStart;
use husky_token::*;
use husky_token_data::{db::TokenDataDb, *};
use parsec::{HasStreamState, StreamParser, TryParseOptionFromStream};
use std::num::NonZeroU32;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenGroupIdx {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RegionalTokenStreamState {
    next_token_idx: RegionalTokenIdx,
    drained: bool,
}

impl RegionalTokenStreamState {
    pub fn next_token_idx(&self) -> RegionalTokenIdx {
        self.next_token_idx
    }

    pub fn drained(&self) -> bool {
        self.drained
    }
}

pub struct RegionalTokenStream<'a> {
    start: RegionalTokenGroupStart,
    tokens: &'a [Token],
    next_relative: RegionalTokenGroupRelativeTokenIndex,
}

impl<'a> HasStreamState for RegionalTokenStream<'a> {
    type State = RegionalTokenStreamState;

    fn save_state(&self) -> Self::State {
        todo!()
    }

    fn rollback(&mut self, state: Self::State) {
        todo!()
    }
}

impl<'a> Iterator for RegionalTokenStream<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<&'a Token> {
        if self.next_relative.index() < self.tokens.len() {
            let next = self.next_relative;
            self.next_relative += 1;
            Some(&self.tokens[next.index()])
        } else {
            None
        }
    }
}

impl<'a> RegionalTokenStream<'a> {
    pub fn is_empty(&self) -> bool {
        self.next_relative.index() >= self.tokens.len()
    }

    pub fn token_position(&self) -> usize {
        self.next_relative.index()
    }

    pub fn try_get_one_token_with_indexed<S>(
        &mut self,
        f: impl Fn(&Token) -> Option<S>,
    ) -> Option<(RegionalTokenIdx, S)> {
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
        assert!(self.next_relative.index() > 0);
        self.next_relative -= 1;
    }

    pub fn rollback(&mut self, state: RegionalTokenIdx) {
        self.next_relative = RegionalTokenGroupRelativeTokenIndex::new(self.start, state);
    }

    pub fn next_index(&self) -> RegionalTokenIdx {
        self.next_relative.regional_token_idx(self.start)
    }

    pub fn next_indexed(&mut self) -> Option<(RegionalTokenIdx, Token)> {
        if self.next_relative.index() < self.tokens.len() {
            let next = self.next_relative;
            self.next_relative += 1;
            Some((
                next.regional_token_idx(self.start),
                self.tokens[next.index()],
            ))
        } else {
            None
        }
    }

    pub fn step_back(&mut self) {
        assert!(self.next_relative.index() > 0);
        self.next_relative -= 1
    }

    pub fn peek(&self) -> Option<&'a Token> {
        if self.next_relative.index() < self.tokens.len() {
            Some(&self.tokens[self.next_relative.index()])
        } else {
            None
        }
    }

    pub fn peek_next_bra(&mut self) -> Option<Bracket> {
        if self.next_relative.index() < self.tokens.len() {
            match self.tokens[self.next_relative.index()] {
                Token::Punctuation(punct) => todo!(),
                //  punct.opt_bra(),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn is_next_ident(&mut self) -> bool {
        match self.peek() {
            Some(token) => match token {
                Token::Ident(_) => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn tokens(&self) -> &[Token] {
        self.tokens
    }

    pub fn rollback_raw(&mut self, token_idx: RegionalTokenIdx) {
        self.next_relative = RegionalTokenGroupRelativeTokenIndex::new(self.start, token_idx)
    }
}

pub trait RegionalTokenStreamParser<'a>:
    HasStreamState<State = RegionalTokenStreamState>
    + StreamParser
    + core::borrow::BorrowMut<RegionalTokenStream<'a>>
{
    fn token_stream(&self) -> &RegionalTokenStream<'a> {
        self.borrow()
    }

    fn token_stream_mut(&mut self) -> &mut RegionalTokenStream<'a> {
        self.borrow_mut()
    }
}

// impl<'a> TokenParseContext<'a> for TokenIter<'a> {}

impl<'a, T> RegionalTokenStreamParser<'a> for T where
    T: HasStreamState<State = RegionalTokenStreamState>
        + StreamParser
        + core::borrow::BorrowMut<RegionalTokenStream<'a>>
{
}

#[cfg(test)]
fn quick_parse<T, Error>(db: &DB, input: &str) -> Result<Option<T>, Error>
where
    T: for<'a> TryParseOptionFromStream<RegionalTokenStream<'a>, Error = Error>,
{
    use husky_vfs::snippet::Snippet;
    use parsec::TryParseOptionFromStream;

    let token_sheet = db.snippet_token_sheet_data(Snippet::new(db, input.to_owned()));
    todo!("get regional token stream")
    // let mut stream = token_sheet
    //     .token_group_token_stream(token_sheet.token_group_iter().next().unwrap().0, None);
    // stream.try_parse_option()
}
