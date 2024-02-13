use super::{idx::*, *};

pub struct TokenVerseIter<'a> {
    tokens: &'a [TokenData],
    verses_data: &'a [TokenVerseData],
    lcurl: Option<TokenIdx>,
    current: usize,
}

impl<'a> TokenVerseIter<'a> {
    pub(crate) fn new(
        tokens: &'a [TokenData],
        verses_data: &'a [TokenVerseData],
        lcurl: Option<TokenIdx>,
    ) -> Self {
        Self {
            tokens,
            verses_data,
            lcurl,
            current: 0,
        }
    }

    pub fn state(&self) -> TokenVerseIdx {
        TokenVerseIdx::new(self.lcurl, self.current)
    }

    pub fn rollback(&mut self, state: TokenVerseIdx) {
        self.current = state.index()
    }

    fn peek(&self) -> Option<(TokenVerseIdx, TokenVerse<'a>)> {
        if self.current >= self.verses_data.len() {
            return None;
        }
        let idx = self.current;
        let verse_data = &self.verses_data[idx];
        let start = verse_data.start();
        let end = self
            .verses_data
            .get(self.current + 1)
            .map(|end| end.start().index())
            .unwrap_or(self.tokens.len()); // tokens already truncated
        Some((
            TokenVerseIdx::new(self.lcurl, idx),
            TokenVerse {
                tokens: &self.tokens[start.index()..end],
                indent: verse_data.indent,
            },
        ))
    }

    pub fn peek_token_verse_of_exact_indent_with_its_first_token(
        &self,
        indent: u32,
    ) -> Option<(TokenVerseIdx, TokenVerse<'a>, TokenData)> {
        let (idx, token_verse) = self.peek()?;
        if token_verse.indent() != indent {
            return None;
        }
        let first_noncomment = token_verse.first();
        Some((idx, token_verse, first_noncomment))
    }

    pub fn next_token_verse_of_no_less_indent_with_its_first_two_tokens(
        &mut self,
        indent: u32,
    ) -> Option<(TokenVerseIdx, TokenVerse<'a>, TokenData, Option<TokenData>)> {
        let (idx, token_verse) = self.peek()?;
        if token_verse.indent() >= indent {
            self.current += 1;
            let fst = token_verse.first();
            let snd = token_verse.second();
            Some((idx, token_verse, fst, snd))
        } else {
            None
        }
    }
}

impl<'a> Iterator for TokenVerseIter<'a> {
    type Item = (TokenVerseIdx, TokenVerse<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.peek()?;
        self.current += 1;
        Some(item)
    }
}
