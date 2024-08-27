use crate::{
    verse::{idx::TokenVerseIdx, start::TokenVerseStart, TokenVerseRelativeTokenIndex},
    TokenIdx, TokenSheetData,
};
use husky_token_data::{Punctuation, TokenData};
#[cfg(test)]
use husky_vfs::script::Script;

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    base: TokenVerseStart,
    tokens: &'a [TokenData],
    next_relative: TokenVerseRelativeTokenIndex,
}

impl TokenSheetData {
    pub fn token_verse_token_stream<'a>(
        &'a self,
        token_verse_idx: TokenVerseIdx,
        saved_stream_state: impl Into<Option<TokenStreamState>>,
    ) -> TokenStream<'a> {
        let state: Option<TokenStreamState> = saved_stream_state.into();
        let base = self.token_verse_start(token_verse_idx);
        let next_relative = state
            .map(|state| TokenVerseRelativeTokenIndex::new(base, state.next_token_idx))
            .unwrap_or_default();
        let tokens = &self[token_verse_idx];
        assert!(tokens.len() > 0);
        TokenStream {
            base,
            tokens,
            next_relative,
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = &'a TokenData;

    fn next(&mut self) -> Option<&'a TokenData> {
        if self.next_relative.index() < self.tokens.len() {
            let next = self.next_relative;
            self.next_relative += 1;
            Some(&self.tokens[next.index()])
        } else {
            None
        }
    }
}

impl<'a> TokenStream<'a> {
    pub fn is_empty(&self) -> bool {
        self.next_relative.index() >= self.tokens.len()
    }

    pub fn token_position(&self) -> usize {
        self.next_relative.index()
    }

    pub fn try_get_one_token_with_indexed<S>(
        &mut self,
        f: impl Fn(&TokenData) -> Option<S>,
    ) -> Option<(TokenIdx, S)> {
        let (token_idx, token) = self.next_indexed()?;
        if let Some(s) = f(&token) {
            Some((token_idx, s))
        } else {
            self.go_back();
            None
        }
    }

    pub fn try_eat_with(
        &mut self,
        predicate: impl FnOnce(&TokenData) -> bool,
    ) -> Option<&'a TokenData> {
        let token = self.peek()?;
        if predicate(&token) {
            self.next();
            Some(token)
        } else {
            None
        }
    }

    pub fn try_eat_special(&mut self, punc: Punctuation) -> Option<&'a TokenData> {
        self.try_eat_with(|token_kind| token_kind == &TokenData::Punctuation(punc))
    }

    pub fn go_back(&mut self) {
        assert!(self.next_relative.index() > 0);
        self.next_relative -= 1;
    }

    pub fn rollback(&mut self, state: TokenIdx) {
        self.next_relative = TokenVerseRelativeTokenIndex::new(self.base, state);
    }

    pub fn next_index(&self) -> TokenIdx {
        self.next_relative.token_idx(self.base)
    }

    pub fn next_indexed(&mut self) -> Option<(TokenIdx, TokenData)> {
        if self.next_relative.index() < self.tokens.len() {
            let next = self.next_relative;
            self.next_relative += 1;
            Some((next.token_idx(self.base), self.tokens[next.index()]))
        } else {
            None
        }
    }

    pub fn step_back(&mut self) {
        assert!(self.next_relative.index() > 0);
        self.next_relative -= 1
    }

    pub fn peek(&self) -> Option<&'a TokenData> {
        if self.next_relative.index() < self.tokens.len() {
            Some(&self.tokens[self.next_relative.index()])
        } else {
            None
        }
    }

    pub fn is_next_ident(&mut self) -> bool {
        match self.peek() {
            Some(token) => match token {
                TokenData::Ident(_) => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn tokens(&self) -> &[TokenData] {
        self.tokens
    }

    pub fn rollback_raw(&mut self, token_idx: TokenIdx) {
        self.next_relative = TokenVerseRelativeTokenIndex::new(self.base, token_idx)
    }
}

#[test]
fn next_indexed_works() {
    use crate::*;

    let db = DB::default();
    let db = &*db;
    let token_sheet_data = db.chunk_token_sheet_data(Script::new_dev_snippet(
        "What does a rusty can of spray-on rust remover smell like?\n Irony.",
        db,
    ));
    let (token_verse_idx, _) = token_sheet_data.main_token_verse_iter().next().unwrap();
    let mut token_iter = token_sheet_data.token_verse_token_stream(token_verse_idx, None);
    while let Some((token_idx, token)) = token_iter.next_indexed() {
        assert_eq!(token_sheet_data[token_idx], token)
    }
}

impl<'a> parsec::HasStreamState for TokenStream<'a> {
    // next_relative
    type State = TokenStreamState;

    fn state(&self) -> Self::State {
        TokenStreamState {
            next_token_idx: self.next_relative.token_idx(self.base),
            drained: self.next_relative.index() >= self.tokens.len(),
        }
    }

    fn rollback(&mut self, state: Self::State) {
        self.rollback_raw(state.next_token_idx)
    }
}

/// this contains extra information than `TokenIdx`
/// - when `drained` is `true`, it indicates a position right before `next_token_idx` and is the end of a token stream (of some token group)
/// - when `drained` is `false`, it indicates the position of the token of index `next_token_idx`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenStreamState {
    next_token_idx: TokenIdx,
    drained: bool,
}

impl TokenStreamState {
    pub unsafe fn new(next_token_idx: TokenIdx, drained: bool) -> Self {
        Self {
            next_token_idx,
            drained,
        }
    }

    pub fn next_token_idx(&self) -> TokenIdx {
        self.next_token_idx
    }

    pub fn drained(&self) -> bool {
        self.drained
    }
}
