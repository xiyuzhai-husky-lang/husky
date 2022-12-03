mod convexity;
mod raw;
mod tokenizer;
mod word;

pub use error::*;

use crate::*;
use convexity::*;
use husky_text::TextCharIter;
use husky_word::WordDb;
use raw::*;
use tokenizer::*;
use word::*;

fn tokenize<'a>(db: &dyn WordDb, char_iter: TextCharIter<'a>) -> Vec<Token> {
    let raw_token_iter = RawTokenIter::new(db, char_iter);
    let mut tokenizer = Tokenizer::new(db);
    tokenizer.push_tokens(raw_token_iter);
    tokenizer.finish_with_tokens()
}

pub trait Tokenize: WordDb {
    fn tokenize(&self, input: &str) -> Vec<Token>;
}

impl<T> Tokenize for T
where
    T: WordDb,
{
    fn tokenize(&self, input: &str) -> Vec<Token> {
        tokenize::tokenize(self, TextCharIter::new(input))
    }
}
