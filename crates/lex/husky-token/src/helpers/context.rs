use parsec::IsStreamParser;

use super::*;

pub trait TokenStreamParser<'a>:
    HasStreamState<State = TokenStreamState> + IsStreamParser + core::borrow::BorrowMut<TokenStream<'a>>
{
    fn token_stream(&self) -> &TokenStream<'a> {
        self.borrow()
    }

    fn token_stream_mut(&mut self) -> &mut TokenStream<'a> {
        self.borrow_mut()
    }
}

// impl<'a> TokenParseContext<'a> for TokenIter<'a> {}

impl<'a, T> TokenStreamParser<'a> for T where
    T: HasStreamState<State = TokenStreamState>
        + IsStreamParser
        + core::borrow::BorrowMut<TokenStream<'a>>
{
}
