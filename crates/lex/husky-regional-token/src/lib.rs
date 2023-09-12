#![feature(trait_upcasting)]
mod ident;
mod keyword;
mod label;
mod path_name;
mod punctuation;
mod symbol_modifier;
#[cfg(test)]
mod tests;

pub use self::ident::*;
pub use self::keyword::*;
pub use self::label::*;
pub use self::path_name::*;
pub use self::punctuation::*;
pub use self::symbol_modifier::*;

#[cfg(test)]
use crate::tests::*;
use husky_coword::Ident;
use husky_token::TokenGroupStartingTokenIdx;
use husky_token::*;
use husky_token_data::{db::TokenDataDb, *};
use parsec::{HasStreamState, StreamParser, TryParseOptionFromStream};
use std::num::NonZeroU32;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdx(NonZeroU32);

impl RegionalTokenIdx {
    pub fn index(self) -> usize {
        self.0.get() as usize - 1
    }

    pub fn token_idx(self, base: TokenRegionBase) -> TokenIdx {
        unsafe { TokenIdx::from_usize_index_ext(self.index() + base.0) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdxRange {
    start: RegionalTokenIdxRangeStart,
    end: RegionalTokenIdxRangeEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdxRangeStart(RegionalTokenIdx);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdxRangeEnd(RegionalTokenIdx);

/// equal to the value of TokenIdx::index on the starting token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenRegionBase(usize);

impl TokenRegionBase {
    pub fn new(token_group_base: TokenGroupStartingTokenIdx) -> Self {
        Self(token_group_base.token_idx().index())
    }
}

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
    tokens: &'a [Token],
}

impl<'a> RegionalTokenStream<'a> {
    pub fn next_indexed(&mut self) -> Option<(RegionalTokenIdx, Token)> {
        todo!()
    }

    pub fn rollback_raw(&mut self, token_idx: RegionalTokenIdx) {
        todo!()
        // self.next_relative = TokenGroupRelativeTokenIndex::new(self.base, token_idx)
    }
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
    use parsec::TryParseOptionFromStream;

    let token_sheet = db.snippet_token_sheet_data(Snippet::new(db, input.to_owned()));
    todo!("get regional token stream")
    // let mut stream = token_sheet
    //     .token_group_token_stream(token_sheet.token_group_iter().next().unwrap().0, None);
    // stream.try_parse_option()
}
