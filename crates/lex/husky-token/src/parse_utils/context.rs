use super::*;

pub trait TokenParseContext<'a>: HasParseState + core::borrow::BorrowMut<TokenStream<'a>> {
    fn token_iter(&self) -> &TokenStream<'a> {
        self.borrow()
    }

    fn token_iter_mut(&mut self) -> &mut TokenStream<'a> {
        self.borrow_mut()
    }
}

// impl<'a> TokenParseContext<'a> for TokenIter<'a> {}

impl<'a, T> TokenParseContext<'a> for T where
    T: HasParseState + core::borrow::BorrowMut<TokenStream<'a>>
{
}
