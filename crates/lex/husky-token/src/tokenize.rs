mod convexity;
mod raw;
mod tokenizer;
mod word;

pub use error::*;

use crate::*;
use convexity::*;
use husky_word::WordDb;
use raw::*;
use tokenizer::*;
use word::*;

// pub trait TokenizeDb: WordDb {
//     fn tokenize_line(&self, line: &str) -> Vec<Token>
//     where
//         Self: Sized,
//     {
//         let mut tokenizer = Tokenizer::new(self);
//         tokenizer.scan_line(0, line);
//         tokenizer.finish_with_tokens()
//     }
// }

// impl<T> TokenizeDb for T where T: WordDb {}
