use self::iter::TokenVerseIter;
use super::*;

#[salsa::debug_with_db]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct MainTokenVerseSequence {
    verses_data: Vec<TokenVerseData>,
}

/// # constructor
impl MainTokenVerseSequence {
    pub(crate) fn new(verses_data: Vec<TokenVerseData>) -> Self {
        Self { verses_data }
    }

    pub(crate) fn token_verse_iter<'a>(&'a self, tokens: &'a [TokenData]) -> TokenVerseIter<'a> {
        TokenVerseIter::new(tokens, &self.verses_data, None)
    }
}

/// # getters
impl MainTokenVerseSequence {
    pub fn verses_data(&self) -> &[TokenVerseData] {
        &self.verses_data
    }
}

/// ```husky
/// |a, b, c| {
///     <nested_token_verse>
///     <nested_token_verse>
///     <nested_token_verse>
/// }
/// ```
/// ```husky
/// |a, b, c| -> i32 = {
///     <nested_token_verse>
///     <nested_token_verse>
///     <nested_token_verse>
/// }
/// ```
// #[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct InlineTokenVerseSequence {
    lcurl: TokenIdx,
    verses_data: Vec<TokenVerseData>,
    end: TokenIdx,
}

/// # getters
impl InlineTokenVerseSequence {
    pub fn lcurl(&self) -> TokenIdx {
        self.lcurl
    }

    pub(crate) fn verses_data(&self) -> &[TokenVerseData] {
        &self.verses_data
    }

    pub fn end(&self) -> TokenIdx {
        self.end
    }
}

impl AsVecMapEntry for InlineTokenVerseSequence {
    type K = TokenIdx;

    fn key_ref(&self) -> &Self::K {
        &self.lcurl
    }
}
