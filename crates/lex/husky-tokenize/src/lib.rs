mod error;
mod raw;
mod scanner;

pub use error::*;

use husky_identifier::IdentifierDb;
use husky_token::Token;
use raw::*;
use scanner::TokenLexer;

pub trait Tokenize: IdentifierDb {
    fn tokenize_line(&self, line: &str) -> Vec<Token> {
        todo!()
        // let mut scanner = TokenLexer::new(self.word_itr());
        // scanner.scan_line(0, line);
        // scanner.finish_with_tokens()
    }
}

impl<T> Tokenize for T where T: IdentifierDb {}

#[cfg(test)]
mod tests {

    use crate::*;
    use husky_expect_test_utils::*;

    #[test]
    fn it_works() {
        expect_test_husky_to_rust("", &tokenize_debug);

        fn tokenize_debug(text: &str) -> String {
            todo!()
            // format!("{:#?}", dyn IdentifierDb::default().tokenize_line(text))
        }
    }
}
