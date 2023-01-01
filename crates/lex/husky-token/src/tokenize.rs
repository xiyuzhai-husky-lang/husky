mod pretoken;
mod reserved;
mod tokenizer;
mod word;

pub use error::*;

use crate::*;
use convexity::*;
use husky_text::TextCharIter;

use pretoken::*;
pub(crate) use reserved::*;
use tokenizer::*;
use word::*;

// #[salsa::tracked(jar = TokenJar)]
pub(crate) fn tokenize<'a>(db: &dyn TokenDb, input: &str) -> Vec<Token> {
    let raw_token_iter = PretokenStream::new(db, TextCharIter::new(input));
    let mut tokenizer = Tokenizer::new(db);
    tokenizer.push_tokens(raw_token_iter);
    tokenizer.finish_with_tokens()
}
