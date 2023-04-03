use parsec::StreamParser;

use super::*;

pub trait TokenParseContext<'a>:
    HasStreamState<State = TokenIdx> + StreamParser + core::borrow::BorrowMut<TokenStream<'a>>
{
    fn token_stream(&self) -> &TokenStream<'a> {
        self.borrow()
    }

    fn token_stream_mut(&mut self) -> &mut TokenStream<'a> {
        self.borrow_mut()
    }
}

// impl<'a> TokenParseContext<'a> for TokenIter<'a> {}

impl<'a, T> TokenParseContext<'a> for T where
    T: HasStreamState<State = TokenIdx> + StreamParser + core::borrow::BorrowMut<TokenStream<'a>>
{
}
