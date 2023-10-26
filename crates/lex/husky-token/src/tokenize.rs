mod pretoken;
mod reserved;
mod tokenizer;
mod word;

pub(crate) use self::reserved::*;

use self::pretoken::*;
use self::tokenizer::*;
use self::word::*;
use crate::*;
use husky_text_protocol::char_iter::TextCharIter;

// must be used inside tracked context
pub(crate) fn tokenize<'a>(db: &dyn TokenDb, input: &str) -> RangedTokenSheet {
    let raw_token_iter = PretokenStream::new(db, TextCharIter::new(input));
    let mut tokenizer = Tokenizer::new(db);
    tokenizer.push_tokens(raw_token_iter);
    tokenizer.finish()
}
