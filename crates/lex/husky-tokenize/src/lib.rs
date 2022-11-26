mod error;
mod raw;
mod tokenizer;
mod word;

pub use error::*;

use husky_identifier::IdentifierDb;
use husky_token::Token;
use raw::*;
use tokenizer::*;
pub trait TokenizeDb: IdentifierDb {
    fn tokenize_line(&self, line: &str) -> Vec<Token>
    where
        Self: Sized,
    {
        let mut tokenizer = Tokenizer::new(self);
        tokenizer.scan_line(0, line);
        tokenizer.finish_with_tokens()
    }
}

impl<T> TokenizeDb for T where T: IdentifierDb {}

#[cfg(test)]
mod tests {

    #[salsa::db(WordJar)]
    #[derive(Default)]
    struct MimicDB {
        storage: Storage<Self>,
    }

    impl Database for MimicDB {}

    use crate::*;
    use husky_expect_test_utils::*;
    use husky_word::WordJar;
    use salsa::{Database, Storage};

    #[test]
    fn tokenize_works() {
        expect_test_husky_to_rust("", &tokenize_debug);

        fn tokenize_debug(text: &str) -> String {
            format!("{:#?}", MimicDB::default().tokenize_line(text))
        }
    }
}
