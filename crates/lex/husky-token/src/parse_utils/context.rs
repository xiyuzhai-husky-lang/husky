use super::*;

pub trait TokenParseContext<'a>: HasParseState {
    fn token_iter(&self) -> &TokenIter<'a>;
    fn token_iter_mut(&mut self) -> &mut TokenIter<'a>;
}

impl<'a> TokenParseContext<'a> for TokenIter<'a> {
    fn token_iter(&self) -> &TokenIter<'a> {
        self
    }

    fn token_iter_mut(&mut self) -> &mut TokenIter<'a> {
        self
    }
}

impl<'a, T> TokenParseContext<'a> for T
where
    T: std::ops::DerefMut<Target = TokenIter<'a>>,
{
    fn token_iter(&self) -> &TokenIter<'a> {
        self.deref()
    }

    fn token_iter_mut(&mut self) -> &mut TokenIter<'a> {
        self.deref_mut()
    }
}
