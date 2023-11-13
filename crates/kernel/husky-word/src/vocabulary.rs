use crate::*;
use compact_str::CompactString;

#[derive(Debug, PartialEq, Eq)]
pub struct Vocabulary {
    // the 0th element is ""
    word_entries: Vec<Entry>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    text: CompactString,
    meanings: smallvec::SmallVec<[WordMeaning; 2]>,
}

#[salsa::tracked(jar = WordJar, return_ref)]
pub(crate) fn vocabulary(_db: &dyn WordDb) -> Vocabulary {
    todo!()
}

impl Vocabulary {
    #[inline(always)]
    pub(crate) fn word_text(&self, word: Word) -> &str {
        &self.word_entries[word.index()].text
    }

    #[inline(always)]
    pub(crate) fn word_meanings(&self, word: Word) -> &[WordMeaning] {
        &self.word_entries[word.index()].meanings
    }
}
