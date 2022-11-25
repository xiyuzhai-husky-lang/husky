use salsa::DbWithJar;

use crate::*;

pub trait WordDb: DbWithJar<WordJar> {
    fn it_word_owned(&self, data: String) -> Word;

    fn it_word_borrowed(&self, data: &str) -> Word;

    fn dt_word(&self, data: Word) -> &str;
}

impl<T> WordDb for T
where
    T: DbWithJar<WordJar>,
{
    fn it_word_owned(&self, data: String) -> Word {
        Word::from_owned(self, data)
    }

    fn it_word_borrowed(&self, data: &str) -> Word {
        Word::from_ref(self, data)
    }

    fn dt_word(&self, word: Word) -> &str {
        word.data(self)
    }
}
