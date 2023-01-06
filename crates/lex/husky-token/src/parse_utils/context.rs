use parsec::{HasParseError, ParseContext};

use super::*;

pub trait TokenParseContext<'a>:
    HasParseState<State = TokenIdx> + ParseContext + core::borrow::BorrowMut<TokenStream<'a>>
where
    <Self as HasParseError>::Error: From<TokenError>,
{
    fn token_stream(&self) -> &TokenStream<'a> {
        self.borrow()
    }

    fn token_stream_mut(&mut self) -> &mut TokenStream<'a> {
        self.borrow_mut()
    }
}

// impl<'a> TokenParseContext<'a> for TokenIter<'a> {}

impl<'a, T> TokenParseContext<'a> for T
where
    T: HasParseState<State = TokenIdx> + ParseContext + core::borrow::BorrowMut<TokenStream<'a>>,
    <Self as HasParseError>::Error: From<TokenError>,
{
}
